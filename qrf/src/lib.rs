#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod qrf_h;
mod sqlite3_h;
use crate::qrf_h::Sqlite3QrfSpec;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs,
};

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct Qrf {
    p_stmt: *mut Sqlite3Stmt,
    db: *mut Sqlite3,
    p_j_trans: *mut Sqlite3Stmt,
    pz_err: *mut *mut i8,
    p_out: *mut Sqlite3Str,
    i_err: i32,
    n_col: i32,
    exp_mode: i32,
    mx_width: i32,
    mx_height: i32,
    u: QrfU0,
    n_row: Sqlite3Int64,
    actual_width: *mut i32,
    spec: Sqlite3QrfSpec,
}

#[repr(C)]
#[derive(Copy, Clone)]
union QrfU0 {
    s_line: QrfU0S0,
    p_graph: *mut QrfEQPGraph,
    s_expln: QrfU0S1,
    n_ins: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct QrfU0S0 {
    mx_col_wth: i32,
    az_col: *mut *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct QrfEQPGraph {
    p_row: *mut QrfEQPGraphRow,
    p_last: *mut QrfEQPGraphRow,
    n_width: i32,
    z_prefix: [i8; 400],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct QrfEQPGraphRow {
    i_eqp_id: i32,
    i_parent_id: i32,
    p_next: *mut QrfEQPGraphRow,
    z_text: [i8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct QrfU0S1 {
    n_indent: i32,
    i_indent: i32,
    ai_indent: *mut i32,
}

///* Set an error code and error message.
unsafe extern "C" fn qrf_error(p: &mut Qrf, i_code_1: i32,
    z_format_1: *const i8, mut __va0: ...) -> () {
    (*p).i_err = i_code_1;
    if (*p).pz_err != core::ptr::null_mut() {
        unsafe { sqlite3_free(unsafe { *(*p).pz_err } as *mut ()) };
        unsafe { *(*p).pz_err = core::ptr::null_mut() };
        if !(z_format_1).is_null() {
            let mut ap: *mut i8 = core::ptr::null_mut();
            unsafe { ap = core::mem::transmute_copy(&__va0) };
            unsafe {
                *(*p).pz_err = unsafe { sqlite3_vmprintf(z_format_1, ap) }
            };
            ();
        }
    }
}

///* Out-of-memory error.
extern "C" fn qrf_oom(p: *mut Qrf) -> () {
    unsafe {
        qrf_error(unsafe { &mut *p }, 7,
            c"out of memory".as_ptr() as *mut i8 as *const i8)
    };
}

///* Initialize the internal Qrf object.
#[allow(unused_doc_comments)]
extern "C" fn qrf_initialize(p: *mut Qrf, p_stmt_1: *mut Sqlite3Stmt,
    p_spec_1: *const Sqlite3QrfSpec, pz_err_1: *mut *mut i8) -> () {
    unsafe {
        let mut sz: u64 = 0 as u64;
        /// Size of pSpec[], based on pSpec->iVersion
        let mut exp_mode: i32 = 0;
        let mut rc: i32 = 0;
        let mut exp_mode_1: i32 = 0;
        let mut rc__1: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s1:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        '__s2:
                            {
                            match unsafe { (*p).spec.e_style } {
                                0 => { __state = 39; }
                                12 => { __state = 40; }
                                10 => { __state = 41; }
                                9 => { __state = 42; }
                                7 => { __state = 43; }
                                8 => { __state = 44; }
                                11 => { __state = 45; }
                                4 => { __state = 46; }
                                15 => { __state = 47; }
                                5 => { __state = 48; }
                                6 => { __state = 49; }
                                _ => { __state = 38; }
                            }
                        }
                    }
                    3 => {
                        unsafe {
                            memset(p as *mut (), 0, core::mem::size_of::<Qrf>() as u64)
                        };
                        __state = 4;
                    }
                    4 => { unsafe { (*p).pz_err = pz_err_1 }; __state = 5; }
                    5 => {
                        if unsafe { (*p_spec_1).i_version } as i32 > 1 {
                            __state = 7;
                        } else { __state = 6; }
                    }
                    6 => { unsafe { (*p).p_stmt = p_stmt_1 }; __state = 9; }
                    7 => {
                        unsafe {
                            qrf_error(unsafe { &mut *p }, 1,
                                c"unusable sqlite3_qrf_spec.iVersion (%d)".as_ptr() as
                                        *mut i8 as *const i8,
                                unsafe { (*p_spec_1).i_version } as i32)
                        };
                        __state = 8;
                    }
                    8 => { return; }
                    9 => {
                        unsafe { (*p).db = unsafe { sqlite3_db_handle(p_stmt_1) } };
                        __state = 10;
                    }
                    10 => {
                        unsafe {
                            (*p).p_out = unsafe { sqlite3_str_new(unsafe { (*p).db }) }
                        };
                        __state = 11;
                    }
                    11 => {
                        if unsafe { (*p).p_out } == core::ptr::null_mut() {
                            __state = 13;
                        } else { __state = 12; }
                    }
                    12 => { unsafe { (*p).i_err = 0 }; __state = 15; }
                    13 => { qrf_oom(p); __state = 14; }
                    14 => { return; }
                    15 => {
                        unsafe {
                            (*p).n_col =
                                unsafe { sqlite3_column_count(unsafe { (*p).p_stmt }) }
                        };
                        __state = 16;
                    }
                    16 => {
                        unsafe { (*p).n_row = 0 as Sqlite3Int64 };
                        __state = 17;
                    }
                    17 => {
                        sz = core::mem::size_of::<Sqlite3QrfSpec>() as u64;
                        __state = 18;
                    }
                    18 => {
                        unsafe {
                            memcpy(unsafe { &raw mut (*p).spec } as *mut (),
                                p_spec_1 as *const (), sz)
                        };
                        __state = 19;
                    }
                    19 => {
                        if unsafe { (*p).spec.z_null } == core::ptr::null_mut() {
                            __state = 21;
                        } else { __state = 20; }
                    }
                    20 => {
                        unsafe {
                            (*p).mx_width = unsafe { (*p).spec.n_screen_width } as i32
                        };
                        __state = 22;
                    }
                    21 => {
                        unsafe { (*p).spec.z_null = c"".as_ptr() as *mut i8 };
                        __state = 20;
                    }
                    22 => {
                        if unsafe { (*p).mx_width } <= 0 {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    23 => {
                        unsafe {
                            (*p).mx_height = unsafe { (*p).spec.n_line_limit } as i32
                        };
                        __state = 25;
                    }
                    24 => {
                        unsafe { (*p).mx_width = 2147483647 };
                        __state = 23;
                    }
                    25 => {
                        if unsafe { (*p).mx_height } <= 0 {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => {
                        if unsafe { (*p).spec.e_style } as i32 > 19 {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    27 => {
                        unsafe { (*p).mx_height = 2147483647 };
                        __state = 26;
                    }
                    28 => {
                        if unsafe { (*p).spec.e_esc } as i32 > 3 {
                            __state = 31;
                        } else { __state = 30; }
                    }
                    29 => {
                        unsafe { (*p).spec.e_style = 0 as u8 };
                        __state = 28;
                    }
                    30 => {
                        if unsafe { (*p).spec.e_text } as i32 > 7 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    31 => {
                        unsafe { (*p).spec.e_esc = 0 as u8 };
                        __state = 30;
                    }
                    32 => {
                        if unsafe { (*p).spec.e_title } as i32 > 7 {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    33 => {
                        unsafe { (*p).spec.e_text = 0 as u8 };
                        __state = 32;
                    }
                    34 => {
                        if unsafe { (*p).spec.e_blob } as i32 > 6 {
                            __state = 37;
                        } else { __state = 36; }
                    }
                    35 => {
                        unsafe { (*p).spec.e_title = 0 as u8 };
                        __state = 34;
                    }
                    36 => { __state = 2; }
                    37 => {
                        unsafe { (*p).spec.e_blob = 0 as u8 };
                        __state = 36;
                    }
                    38 => {
                        if unsafe { (*p).spec.e_esc } as i32 == 0 {
                            __state = 108;
                        } else { __state = 107; }
                    }
                    39 => {
                        '__s3:
                            {
                            match unsafe { sqlite3_stmt_isexplain(p_stmt_1) } {
                                0 => { __state = 53; }
                                1 => { __state = 54; }
                                _ => { __state = 55; }
                            }
                        }
                    }
                    40 => {
                        if unsafe { (*p).spec.z_column_sep } ==
                                core::ptr::null_mut() {
                            __state = 64;
                        } else { __state = 63; }
                    }
                    41 => { __state = 42; }
                    42 => {
                        unsafe { (*p).spec.e_text = 6 as u8 };
                        __state = 68;
                    }
                    43 => {
                        unsafe { (*p).spec.e_text = 4 as u8 };
                        __state = 72;
                    }
                    44 => {
                        unsafe { (*p).spec.e_text = 2 as u8 };
                        __state = 75;
                    }
                    45 => {
                        if unsafe { (*p).spec.z_column_sep } ==
                                core::ptr::null_mut() {
                            __state = 82;
                        } else { __state = 81; }
                    }
                    46 => {
                        unsafe { (*p).spec.e_style = 12 as u8 };
                        __state = 84;
                    }
                    47 => {
                        unsafe { (*p).spec.e_text = 2 as u8 };
                        __state = 90;
                    }
                    48 => {
                        exp_mode =
                            unsafe { sqlite3_stmt_isexplain(unsafe { (*p).p_stmt }) };
                        __state = 95;
                    }
                    49 => {
                        exp_mode_1 =
                            unsafe { sqlite3_stmt_isexplain(unsafe { (*p).p_stmt }) };
                        __state = 101;
                    }
                    50 => { __state = 39; }
                    51 => { __state = 40; }
                    52 => { __state = 2; }
                    53 => {
                        unsafe { (*p).spec.e_style = 1 as u8 };
                        __state = 57;
                    }
                    54 => {
                        unsafe { (*p).spec.e_style = 6 as u8 };
                        __state = 59;
                    }
                    55 => {
                        unsafe { (*p).spec.e_style = 5 as u8 };
                        __state = 61;
                    }
                    56 => { __state = 53; }
                    57 => { __state = 52; }
                    58 => { __state = 54; }
                    59 => { __state = 52; }
                    60 => { __state = 55; }
                    61 => { __state = 52; }
                    62 => { __state = 41; }
                    63 => {
                        if unsafe { (*p).spec.z_row_sep } == core::ptr::null_mut() {
                            __state = 66;
                        } else { __state = 65; }
                    }
                    64 => {
                        unsafe {
                            (*p).spec.z_column_sep = c"|".as_ptr() as *mut i8
                        };
                        __state = 63;
                    }
                    65 => { __state = 38; }
                    66 => {
                        unsafe { (*p).spec.z_row_sep = c"\n".as_ptr() as *mut i8 };
                        __state = 65;
                    }
                    67 => { __state = 70; }
                    68 => {
                        unsafe { (*p).spec.z_null = c"null".as_ptr() as *mut i8 };
                        __state = 69;
                    }
                    69 => { __state = 38; }
                    70 => { __state = 43; }
                    71 => { __state = 44; }
                    72 => {
                        unsafe { (*p).spec.z_null = c"null".as_ptr() as *mut i8 };
                        __state = 73;
                    }
                    73 => { __state = 38; }
                    74 => { __state = 45; }
                    75 => {
                        unsafe { (*p).spec.z_null = c"NULL".as_ptr() as *mut i8 };
                        __state = 76;
                    }
                    76 => {
                        if unsafe { (*p).spec.z_table_name } ==
                                    core::ptr::null_mut() ||
                                unsafe {
                                            *unsafe { (*p).spec.z_table_name.offset(0 as isize) }
                                        } as i32 == 0 {
                            __state = 78;
                        } else { __state = 77; }
                    }
                    77 => { unsafe { (*p).u.n_ins = 0 as u32 }; __state = 79; }
                    78 => {
                        unsafe {
                            (*p).spec.z_table_name = c"tab".as_ptr() as *mut i8
                        };
                        __state = 77;
                    }
                    79 => { __state = 38; }
                    80 => { __state = 46; }
                    81 => { __state = 38; }
                    82 => {
                        unsafe {
                            (*p).spec.z_column_sep = c": ".as_ptr() as *mut i8
                        };
                        __state = 81;
                    }
                    83 => { __state = 47; }
                    84 => {
                        unsafe { (*p).spec.e_text = 3 as u8 };
                        __state = 85;
                    }
                    85 => {
                        unsafe {
                            (*p).spec.z_column_sep = c",".as_ptr() as *mut i8
                        };
                        __state = 86;
                    }
                    86 => {
                        unsafe {
                            (*p).spec.z_row_sep = c"\r\n".as_ptr() as *mut i8
                        };
                        __state = 87;
                    }
                    87 => {
                        unsafe { (*p).spec.z_null = c"".as_ptr() as *mut i8 };
                        __state = 88;
                    }
                    88 => { __state = 38; }
                    89 => { __state = 48; }
                    90 => {
                        unsafe { (*p).spec.z_null = c"NULL".as_ptr() as *mut i8 };
                        __state = 91;
                    }
                    91 => {
                        unsafe {
                            (*p).spec.z_column_sep = c",".as_ptr() as *mut i8
                        };
                        __state = 92;
                    }
                    92 => {
                        unsafe { (*p).spec.z_row_sep = c"\n".as_ptr() as *mut i8 };
                        __state = 93;
                    }
                    93 => { __state = 38; }
                    94 => { __state = 49; }
                    95 => {
                        if exp_mode != 2 { __state = 97; } else { __state = 96; }
                    }
                    96 => { __state = 38; }
                    97 => {
                        rc =
                            unsafe { sqlite3_stmt_explain(unsafe { (*p).p_stmt }, 2) };
                        __state = 98;
                    }
                    98 => {
                        if rc != 0 { __state = 100; } else { __state = 99; }
                    }
                    99 => {
                        unsafe { (*p).exp_mode = exp_mode + 1 };
                        __state = 96;
                    }
                    100 => {
                        unsafe {
                            qrf_error(unsafe { &mut *p }, 1,
                                unsafe { sqlite3_errstr(rc) })
                        };
                        __state = 99;
                    }
                    101 => {
                        if exp_mode_1 != 1 {
                            __state = 103;
                        } else { __state = 102; }
                    }
                    102 => { __state = 38; }
                    103 => {
                        rc__1 =
                            unsafe { sqlite3_stmt_explain(unsafe { (*p).p_stmt }, 1) };
                        __state = 104;
                    }
                    104 => {
                        if rc__1 != 0 { __state = 106; } else { __state = 105; }
                    }
                    105 => {
                        unsafe { (*p).exp_mode = exp_mode_1 + 1 };
                        __state = 102;
                    }
                    106 => {
                        unsafe {
                            qrf_error(unsafe { &mut *p }, 1,
                                unsafe { sqlite3_errstr(rc__1) })
                        };
                        __state = 105;
                    }
                    107 => {
                        if unsafe { (*p).spec.e_text } as i32 == 0 {
                            __state = 110;
                        } else { __state = 109; }
                    }
                    108 => {
                        unsafe { (*p).spec.e_esc = 2 as u8 };
                        __state = 107;
                    }
                    109 => {
                        if unsafe { (*p).spec.e_title } as i32 == 0 {
                            __state = 112;
                        } else { __state = 111; }
                    }
                    110 => {
                        unsafe { (*p).spec.e_text = 1 as u8 };
                        __state = 109;
                    }
                    111 => {
                        if unsafe { (*p).spec.e_blob } as i32 == 0 {
                            __state = 122;
                        } else { __state = 121; }
                    }
                    112 => {
                        '__s4:
                            {
                            match unsafe { (*p).spec.e_style } {
                                1 => { __state = 113; }
                                2 => { __state = 114; }
                                19 => { __state = 115; }
                                _ => { __state = 116; }
                            }
                        }
                    }
                    113 => { __state = 114; }
                    114 => { __state = 115; }
                    115 => {
                        unsafe { (*p).spec.e_title = 1 as u8 };
                        __state = 118;
                    }
                    116 => {
                        unsafe { (*p).spec.e_title = unsafe { (*p).spec.e_text } };
                        __state = 120;
                    }
                    117 => { __state = 113; }
                    118 => { __state = 111; }
                    119 => { __state = 116; }
                    120 => { __state = 111; }
                    121 => {
                        if unsafe { (*p).spec.b_titles } as i32 == 0 {
                            __state = 139;
                        } else { __state = 138; }
                    }
                    122 => {
                        '__s5:
                            {
                            match unsafe { (*p).spec.e_text } {
                                2 => { __state = 123; }
                                3 => { __state = 124; }
                                5 => { __state = 125; }
                                6 => { __state = 126; }
                                _ => { __state = 127; }
                            }
                        }
                    }
                    123 => {
                        unsafe { (*p).spec.e_blob = 2 as u8 };
                        __state = 129;
                    }
                    124 => {
                        unsafe { (*p).spec.e_blob = 4 as u8 };
                        __state = 131;
                    }
                    125 => {
                        unsafe { (*p).spec.e_blob = 4 as u8 };
                        __state = 133;
                    }
                    126 => {
                        unsafe { (*p).spec.e_blob = 5 as u8 };
                        __state = 135;
                    }
                    127 => {
                        unsafe { (*p).spec.e_blob = 1 as u8 };
                        __state = 137;
                    }
                    128 => { __state = 123; }
                    129 => { __state = 121; }
                    130 => { __state = 124; }
                    131 => { __state = 121; }
                    132 => { __state = 125; }
                    133 => { __state = 121; }
                    134 => { __state = 126; }
                    135 => { __state = 121; }
                    136 => { __state = 127; }
                    137 => { __state = 121; }
                    138 => {
                        if unsafe { (*p).spec.b_word_wrap } as i32 == 0 {
                            __state = 151;
                        } else { __state = 150; }
                    }
                    139 => {
                        '__s6:
                            {
                            match unsafe { (*p).spec.e_style } {
                                1 => { __state = 140; }
                                4 => { __state = 141; }
                                2 => { __state = 142; }
                                19 => { __state = 143; }
                                13 => { __state = 144; }
                                _ => { __state = 145; }
                            }
                        }
                    }
                    140 => { __state = 141; }
                    141 => { __state = 142; }
                    142 => { __state = 143; }
                    143 => { __state = 144; }
                    144 => {
                        unsafe { (*p).spec.b_titles = 2 as u8 };
                        __state = 147;
                    }
                    145 => {
                        unsafe { (*p).spec.b_titles = 1 as u8 };
                        __state = 149;
                    }
                    146 => { __state = 140; }
                    147 => { __state = 138; }
                    148 => { __state = 145; }
                    149 => { __state = 138; }
                    150 => {
                        if unsafe { (*p).spec.b_text_jsonb } as i32 == 0 {
                            __state = 153;
                        } else { __state = 152; }
                    }
                    151 => {
                        unsafe { (*p).spec.b_word_wrap = 2 as u8 };
                        __state = 150;
                    }
                    152 => {
                        if unsafe { (*p).spec.z_column_sep } ==
                                core::ptr::null_mut() {
                            __state = 155;
                        } else { __state = 154; }
                    }
                    153 => {
                        unsafe { (*p).spec.b_text_jsonb = 1 as u8 };
                        __state = 152;
                    }
                    154 => {
                        if unsafe { (*p).spec.z_row_sep } == core::ptr::null_mut() {
                            __state = 156;
                        } else { __state = 1; }
                    }
                    155 => {
                        unsafe {
                            (*p).spec.z_column_sep = c",".as_ptr() as *mut i8
                        };
                        __state = 154;
                    }
                    156 => {
                        unsafe { (*p).spec.z_row_sep = c"\n".as_ptr() as *mut i8 };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct QrfColData {
    p: *mut Qrf,
    n_col: i32,
    b_multi_row: u8,
    n_margin: u8,
    n_row: Sqlite3Int64,
    n_alloc: Sqlite3Int64,
    n: Sqlite3Int64,
    az: *mut *mut i8,
    ai_wth: *mut i32,
    ab_num: *mut u8,
    a: *mut QrfPerCol,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct QrfPerCol {
    z: *mut i8,
    w: i32,
    mx_w: i32,
    e: u8,
    fx: u8,
    b_num: u8,
}

///* Free all the memory allocates in the qrfColData object
extern "C" fn qrf_col_data_free(p: *mut QrfColData) -> () {
    let mut i: Sqlite3Int64 = 0 as Sqlite3Int64;
    {
        i = 0 as Sqlite3Int64;
        '__b7: loop {
            if !(i < unsafe { (*p).n }) { break '__b7; }
            '__c7: loop {
                unsafe {
                    sqlite3_free(unsafe {
                                *unsafe { (*p).az.offset(i as isize) }
                            } as *mut ())
                };
                break '__c7;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p).az } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).ai_wth } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).ab_num } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).a } as *mut ()) };
    unsafe {
        memset(p as *mut (), 0, core::mem::size_of::<QrfColData>() as u64)
    };
}

///* Allocate space for more cells in the qrfColData object.
///* Return non-zero if a memory allocation fails.
extern "C" fn qrf_col_data_enlarge(p: *mut QrfColData) -> i32 {
    let mut az_data: *mut *mut i8 = core::ptr::null_mut();
    let mut ai_wth: *mut i32 = core::ptr::null_mut();
    let mut ab_num: *mut u8 = core::ptr::null_mut();
    unsafe {
        (*p).n_alloc =
            2 as Sqlite3Int64 * unsafe { (*p).n_alloc } +
                (10 * unsafe { (*p).n_col }) as Sqlite3Int64
    };
    az_data =
        unsafe {
                sqlite3_realloc64(unsafe { (*p).az } as *mut (),
                    unsafe { (*p).n_alloc } as u64 *
                        core::mem::size_of::<*mut i8>() as u64)
            } as *mut *mut i8;
    if az_data == core::ptr::null_mut() {
        qrf_oom(unsafe { (*p).p });
        qrf_col_data_free(p);
        return 1;
    }
    unsafe { (*p).az = az_data };
    ai_wth =
        unsafe {
                sqlite3_realloc64(unsafe { (*p).ai_wth } as *mut (),
                    unsafe { (*p).n_alloc } as u64 *
                        core::mem::size_of::<i32>() as u64)
            } as *mut i32;
    if ai_wth == core::ptr::null_mut() {
        qrf_oom(unsafe { (*p).p });
        qrf_col_data_free(p);
        return 1;
    }
    unsafe { (*p).ai_wth = ai_wth };
    ab_num =
        unsafe {
                sqlite3_realloc64(unsafe { (*p).ab_num } as *mut (),
                    unsafe { (*p).n_alloc } as Sqlite3Uint64)
            } as *mut u8;
    if ab_num == core::ptr::null_mut() {
        qrf_oom(unsafe { (*p).p });
        qrf_col_data_free(p);
        return 1;
    }
    unsafe { (*p).ab_num = ab_num };
    return 0;
}

///* Data for substitute ctype.h functions.  Used for x-platform
///* consistency and so that '_' is counted as an alphabetic
///* character.
///*
///*    0x01 -  space
///*    0x02 -  digit
///*    0x04 -  alphabetic, including '_'
static qrf_c_type: [i8; 256] =
    [0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 1 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 4 as i8, 0 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8];

///* Determine if the string z[] can be shown as plain text.  Return true
///* if z[] is unambiguously text.  Return false if z[] needs to be
///* quoted.
///*
///* All of the following must be true in order for z[] to be relaxable:
///*
///*    (1) z[] does not begin or end with ' or whitespace
///*    (2) z[] is not the same as the NULL rendering
///*    (3) z[] does not looks like a numeric literal
extern "C" fn qrf_relaxable(p: &Qrf, z: *const i8) -> i32 {
    let mut i: u64 = 0 as u64;
    let mut n: u64 = 0 as u64;
    if unsafe { *z.offset(0 as isize) } as i32 == '\'' as i32 ||
            qrf_c_type[unsafe { *z.offset(0 as isize) } as u8 as usize] as i32
                    & 1 != 0 {
        return 0;
    }
    if unsafe { *z.offset(0 as isize) } as i32 == 0 {
        return ((*p).spec.z_null != core::ptr::null_mut() &&
                    unsafe { *(*p).spec.z_null.offset(0 as isize) } as i32 != 0)
                as i32;
    }
    n = unsafe { strlen(z) };
    if n == 0 as u64 ||
                unsafe { *z.add((n - 1 as u64) as usize) } as i32 ==
                    '\'' as i32 ||
            qrf_c_type[unsafe { *z.add((n - 1 as u64) as usize) } as u8 as
                                usize] as i32 & 1 != 0 {
        return 0;
    }
    if !((*p).spec.z_null).is_null() &&
            unsafe { strcmp((*p).spec.z_null as *const i8, z) } == 0 {
        return 0;
    }
    i =
        (unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 ||
                unsafe { *z.offset(0 as isize) } as i32 == '+' as i32) as u64;
    if unsafe {
                strcmp(unsafe { z.add(i as usize) },
                    c"Inf".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 0;
    }
    if !(qrf_c_type[unsafe { *z.add(i as usize) } as u8 as usize] as i32 & 2
                        != 0) as i32 != 0 {
        return 1;
    }
    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    while qrf_c_type[unsafe { *z.add(i as usize) } as u8 as usize] as i32 & 2
            != 0 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }
    if unsafe { *z.add(i as usize) } as i32 == 0 { return 0; }
    if unsafe { *z.add(i as usize) } as i32 == '.' as i32 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        while qrf_c_type[unsafe { *z.add(i as usize) } as u8 as usize] as i32
                    & 2 != 0 {
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
        if unsafe { *z.add(i as usize) } as i32 == 0 { return 0; }
    }
    if unsafe { *z.add(i as usize) } as i32 == 'e' as i32 ||
            unsafe { *z.add(i as usize) } as i32 == 'E' as i32 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        if unsafe { *z.add(i as usize) } as i32 == '+' as i32 ||
                unsafe { *z.add(i as usize) } as i32 == '-' as i32 {
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
        if !(qrf_c_type[unsafe { *z.add(i as usize) } as u8 as usize] as i32 &
                                2 != 0) as i32 != 0 {
            return 1;
        }
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        while qrf_c_type[unsafe { *z.add(i as usize) } as u8 as usize] as i32
                    & 2 != 0 {
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return (unsafe { *z.add(i as usize) } as i32 != 0) as i32;
}

///* If a field contains any character identified by a 1 in the following
///* array, then the string must be quoted for CSV.
static qrf_csv_quote: [i8; 256] =
    [1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 0 as i8, 1 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 1 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8];

///* Escape the text starting at byte iStart of pStr, if needed, using the
///* escape encoding of eEsc, which is either QRF_ESC_Ascii or QRF_ESC_Symbol.
///* The pStr string is modified appropriately.
///*
///* Escaping is needed if the string contains any control characters
///* other than \t, \n, and \r\n
///*
///* If no escaping is needed (the common case) then pStr is unchanged.
///* If escaping is required, then pStr is expanded and modified to hold
///* an escaped representation of the text.
#[allow(unused_doc_comments)]
extern "C" fn qrf_escape(e_esc_1: i32, p_str_1: *mut Sqlite3Str,
    i_start_1: i32) -> () {
    let mut i: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut j: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Loop counters
    let mut n_ctrl: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Number of control characters to escape
    let mut z_in: *mut u8 = core::ptr::null_mut();
    /// Text to be escaped
    let mut n_in: u32 = 0 as u32;
    /// Bytes of text to be escaped
    let mut c: u8 = 0 as u8;
    /// A single character of the text
    let mut z_out: *mut u8 = core::ptr::null_mut();

    /// Where to write the results
    /// Find the text to be escaped
    (z_in = unsafe { sqlite3_str_value(p_str_1) } as *mut u8);

    /// Where to write the results
    /// Find the text to be escaped
    (n_in = unsafe { sqlite3_str_length(p_str_1) } as u32);
    if z_in == core::ptr::null_mut() { return; }
    {
        let __n = i_start_1;
        let __p = &mut z_in;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    n_in -= i_start_1 as u32;
    {
        i = 0 as Sqlite3Int64;
        '__b11: loop {
            if !(i < n_in as i64) { break '__b11; }
            '__c11: loop {
                if { c = unsafe { *z_in.offset(i as isize) }; c } as i32 <= 31
                                && c as i32 != '\t' as i32 && c as i32 != '\n' as i32 &&
                        (c as i32 != '\r' as i32 ||
                            unsafe { *z_in.offset((i + 1 as Sqlite3Int64) as isize) } as
                                    i32 != '\n' as i32) {
                    { let __p = &mut n_ctrl; let __t = *__p; *__p += 1; __t };
                }
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_ctrl == 0 as i64 { return; }
    if e_esc_1 == 3 { n_ctrl *= 2 as Sqlite3Int64; }
    unsafe {
        sqlite3_str_appendchar(p_str_1, n_ctrl as i32, ' ' as i32 as i8)
    };
    z_out = unsafe { sqlite3_str_value(p_str_1) } as *mut u8;
    if z_out == core::ptr::null_mut() { return; }
    {
        let __n = i_start_1;
        let __p = &mut z_out;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    z_in = unsafe { z_out.offset(n_ctrl as isize) };
    unsafe { memmove(z_in as *mut (), z_out as *const (), n_in as u64) };
    {
        i = { j = 0 as Sqlite3Int64; j };
        '__b12: loop {
            if !(i < n_in as i64) { break '__b12; }
            '__c12: loop {
                if { c = unsafe { *z_in.offset(i as isize) }; c } as i32 > 31
                                || c as i32 == '\t' as i32 || c as i32 == '\n' as i32 ||
                        c as i32 == '\r' as i32 &&
                            unsafe { *z_in.offset((i + 1 as Sqlite3Int64) as isize) } as
                                    i32 == '\n' as i32 {
                    break '__c12;
                }
                if i > 0 as i64 {
                    unsafe {
                        memmove(unsafe { &raw mut *z_out.offset(j as isize) } as
                                *mut (), z_in as *const (), i as u64)
                    };
                    j += i;
                }
                {
                    let __n = i + 1 as Sqlite3Int64;
                    let __p = &mut z_in;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                n_in -= (i + 1 as Sqlite3Int64) as u32;
                i = -1 as Sqlite3Int64;
                if e_esc_1 == 3 {
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = 226 as u8
                    };
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = 144 as u8
                    };
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = (128 + c as i32) as u8
                    };
                } else {
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = '^' as i32 as u8
                    };
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = (64 + c as i32) as u8
                    };
                }
                break '__c12;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Encode text appropriately and append it to pOut.
extern "C" fn qrf_encode_text(p: *mut Qrf, p_out_1: *mut Sqlite3Str,
    z_txt_1: *const i8) -> () {
    let i_start: i32 = unsafe { sqlite3_str_length(p_out_1) };
    '__s13:
        {
        match unsafe { (*p).spec.e_text } {
            7 => {
                if qrf_relaxable(unsafe { &*p }, z_txt_1) != 0 {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
                {
                    if unsafe { (*p).spec.e_esc } as i32 == 1 {
                        unsafe {
                            sqlite3_str_appendf(p_out_1,
                                c"%Q".as_ptr() as *mut i8 as *const i8, z_txt_1)
                        };
                    } else {
                        unsafe {
                            sqlite3_str_appendf(p_out_1,
                                c"%#Q".as_ptr() as *mut i8 as *const i8, z_txt_1)
                        };
                    }
                    break '__s13;
                }
                {
                    let mut i: u32 = 0 as u32;
                    {
                        i = 0 as u32;
                        '__b14: loop {
                            if !(unsafe { *z_txt_1.add(i as usize) } != 0) {
                                break '__b14;
                            }
                            '__c14: loop {
                                if qrf_csv_quote[unsafe {
                                                    *(z_txt_1 as *const u8).add(i as usize)
                                                } as usize] != 0 {
                                    i = 0 as u32;
                                    break '__b14;
                                }
                                break '__c14;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if i == 0 as u32 ||
                            unsafe {
                                    strstr(z_txt_1,
                                        unsafe { (*p).spec.z_column_sep } as *const i8)
                                } != core::ptr::null_mut() {
                        unsafe {
                            sqlite3_str_appendf(p_out_1,
                                c"\"%w\"".as_ptr() as *mut i8 as *const i8, z_txt_1)
                        };
                    } else {
                        unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    }
                    break '__s13;
                }
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        let mut c: u8 = 0 as u8;
                        while { c = unsafe { *z.add(i as usize) } as u8; c } as i32
                                    > '>' as i32 ||
                                c != 0 && c as i32 != '<' as i32 && c as i32 != '>' as i32
                                            && c as i32 != '&' as i32 && c as i32 != '\"' as i32 &&
                                    c as i32 != '\'' as i32 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        '__s17:
                            {
                            match unsafe { *z.add(i as usize) } {
                                62 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&gt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                38 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&amp;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                60 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&lt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&quot;".as_ptr() as *mut i8 as *const i8, 6)
                                    };
                                }
                                39 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&#39;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                _ => {
                                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    break '__s13;
                }
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        {
                            i = 0 as u32;
                            '__b19: loop {
                                if !(unsafe { *z.add(i as usize) } as i32 >= 32 &&
                                                    unsafe { *z.add(i as usize) } as i32 != '\\' as i32 &&
                                                unsafe { *z.add(i as usize) } as i32 != '\"' as i32) {
                                    break '__b19;
                                }
                                '__c19: loop { break '__c19; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        if unsafe { *z.add(i as usize) } as i32 == 0 { break; }
                        '__s20:
                            {
                            match unsafe { *z.add(i as usize) } {
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\"".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                92 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\\".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                8 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\b".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                12 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\f".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                10 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\n".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                13 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\r".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                9 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\t".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                _ => {
                                    {
                                        if unsafe { (*p).spec.e_text } as i32 == 6 {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\u%04x".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        } else {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\%03o".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        }
                                        break '__s20;
                                    }
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    break '__s13;
                }
                {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
            }
            2 => {
                {
                    if unsafe { (*p).spec.e_esc } as i32 == 1 {
                        unsafe {
                            sqlite3_str_appendf(p_out_1,
                                c"%Q".as_ptr() as *mut i8 as *const i8, z_txt_1)
                        };
                    } else {
                        unsafe {
                            sqlite3_str_appendf(p_out_1,
                                c"%#Q".as_ptr() as *mut i8 as *const i8, z_txt_1)
                        };
                    }
                    break '__s13;
                }
                {
                    let mut i: u32 = 0 as u32;
                    {
                        i = 0 as u32;
                        '__b14: loop {
                            if !(unsafe { *z_txt_1.add(i as usize) } != 0) {
                                break '__b14;
                            }
                            '__c14: loop {
                                if qrf_csv_quote[unsafe {
                                                    *(z_txt_1 as *const u8).add(i as usize)
                                                } as usize] != 0 {
                                    i = 0 as u32;
                                    break '__b14;
                                }
                                break '__c14;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if i == 0 as u32 ||
                            unsafe {
                                    strstr(z_txt_1,
                                        unsafe { (*p).spec.z_column_sep } as *const i8)
                                } != core::ptr::null_mut() {
                        unsafe {
                            sqlite3_str_appendf(p_out_1,
                                c"\"%w\"".as_ptr() as *mut i8 as *const i8, z_txt_1)
                        };
                    } else {
                        unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    }
                    break '__s13;
                }
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        let mut c: u8 = 0 as u8;
                        while { c = unsafe { *z.add(i as usize) } as u8; c } as i32
                                    > '>' as i32 ||
                                c != 0 && c as i32 != '<' as i32 && c as i32 != '>' as i32
                                            && c as i32 != '&' as i32 && c as i32 != '\"' as i32 &&
                                    c as i32 != '\'' as i32 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        '__s17:
                            {
                            match unsafe { *z.add(i as usize) } {
                                62 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&gt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                38 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&amp;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                60 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&lt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&quot;".as_ptr() as *mut i8 as *const i8, 6)
                                    };
                                }
                                39 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&#39;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                _ => {
                                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    break '__s13;
                }
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        {
                            i = 0 as u32;
                            '__b19: loop {
                                if !(unsafe { *z.add(i as usize) } as i32 >= 32 &&
                                                    unsafe { *z.add(i as usize) } as i32 != '\\' as i32 &&
                                                unsafe { *z.add(i as usize) } as i32 != '\"' as i32) {
                                    break '__b19;
                                }
                                '__c19: loop { break '__c19; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        if unsafe { *z.add(i as usize) } as i32 == 0 { break; }
                        '__s20:
                            {
                            match unsafe { *z.add(i as usize) } {
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\"".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                92 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\\".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                8 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\b".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                12 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\f".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                10 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\n".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                13 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\r".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                9 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\t".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                _ => {
                                    {
                                        if unsafe { (*p).spec.e_text } as i32 == 6 {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\u%04x".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        } else {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\%03o".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        }
                                        break '__s20;
                                    }
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    break '__s13;
                }
                {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
            }
            3 => {
                {
                    let mut i: u32 = 0 as u32;
                    {
                        i = 0 as u32;
                        '__b14: loop {
                            if !(unsafe { *z_txt_1.add(i as usize) } != 0) {
                                break '__b14;
                            }
                            '__c14: loop {
                                if qrf_csv_quote[unsafe {
                                                    *(z_txt_1 as *const u8).add(i as usize)
                                                } as usize] != 0 {
                                    i = 0 as u32;
                                    break '__b14;
                                }
                                break '__c14;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if i == 0 as u32 ||
                            unsafe {
                                    strstr(z_txt_1,
                                        unsafe { (*p).spec.z_column_sep } as *const i8)
                                } != core::ptr::null_mut() {
                        unsafe {
                            sqlite3_str_appendf(p_out_1,
                                c"\"%w\"".as_ptr() as *mut i8 as *const i8, z_txt_1)
                        };
                    } else {
                        unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    }
                    break '__s13;
                }
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        let mut c: u8 = 0 as u8;
                        while { c = unsafe { *z.add(i as usize) } as u8; c } as i32
                                    > '>' as i32 ||
                                c != 0 && c as i32 != '<' as i32 && c as i32 != '>' as i32
                                            && c as i32 != '&' as i32 && c as i32 != '\"' as i32 &&
                                    c as i32 != '\'' as i32 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        '__s17:
                            {
                            match unsafe { *z.add(i as usize) } {
                                62 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&gt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                38 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&amp;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                60 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&lt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&quot;".as_ptr() as *mut i8 as *const i8, 6)
                                    };
                                }
                                39 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&#39;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                _ => {
                                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    break '__s13;
                }
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        {
                            i = 0 as u32;
                            '__b19: loop {
                                if !(unsafe { *z.add(i as usize) } as i32 >= 32 &&
                                                    unsafe { *z.add(i as usize) } as i32 != '\\' as i32 &&
                                                unsafe { *z.add(i as usize) } as i32 != '\"' as i32) {
                                    break '__b19;
                                }
                                '__c19: loop { break '__c19; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        if unsafe { *z.add(i as usize) } as i32 == 0 { break; }
                        '__s20:
                            {
                            match unsafe { *z.add(i as usize) } {
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\"".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                92 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\\".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                8 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\b".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                12 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\f".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                10 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\n".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                13 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\r".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                9 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\t".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                _ => {
                                    {
                                        if unsafe { (*p).spec.e_text } as i32 == 6 {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\u%04x".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        } else {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\%03o".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        }
                                        break '__s20;
                                    }
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    break '__s13;
                }
                {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
            }
            4 => {
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        let mut c: u8 = 0 as u8;
                        while { c = unsafe { *z.add(i as usize) } as u8; c } as i32
                                    > '>' as i32 ||
                                c != 0 && c as i32 != '<' as i32 && c as i32 != '>' as i32
                                            && c as i32 != '&' as i32 && c as i32 != '\"' as i32 &&
                                    c as i32 != '\'' as i32 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        '__s17:
                            {
                            match unsafe { *z.add(i as usize) } {
                                62 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&gt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                38 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&amp;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                60 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&lt;".as_ptr() as *mut i8 as *const i8, 4)
                                    };
                                }
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&quot;".as_ptr() as *mut i8 as *const i8, 6)
                                    };
                                }
                                39 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"&#39;".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                }
                                _ => {
                                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    break '__s13;
                }
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        {
                            i = 0 as u32;
                            '__b19: loop {
                                if !(unsafe { *z.add(i as usize) } as i32 >= 32 &&
                                                    unsafe { *z.add(i as usize) } as i32 != '\\' as i32 &&
                                                unsafe { *z.add(i as usize) } as i32 != '\"' as i32) {
                                    break '__b19;
                                }
                                '__c19: loop { break '__c19; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        if unsafe { *z.add(i as usize) } as i32 == 0 { break; }
                        '__s20:
                            {
                            match unsafe { *z.add(i as usize) } {
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\"".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                92 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\\".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                8 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\b".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                12 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\f".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                10 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\n".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                13 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\r".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                9 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\t".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                _ => {
                                    {
                                        if unsafe { (*p).spec.e_text } as i32 == 6 {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\u%04x".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        } else {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\%03o".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        }
                                        break '__s20;
                                    }
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    break '__s13;
                }
                {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
            }
            5 => {
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        {
                            i = 0 as u32;
                            '__b19: loop {
                                if !(unsafe { *z.add(i as usize) } as i32 >= 32 &&
                                                    unsafe { *z.add(i as usize) } as i32 != '\\' as i32 &&
                                                unsafe { *z.add(i as usize) } as i32 != '\"' as i32) {
                                    break '__b19;
                                }
                                '__c19: loop { break '__c19; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        if unsafe { *z.add(i as usize) } as i32 == 0 { break; }
                        '__s20:
                            {
                            match unsafe { *z.add(i as usize) } {
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\"".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                92 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\\".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                8 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\b".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                12 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\f".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                10 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\n".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                13 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\r".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                9 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\t".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                _ => {
                                    {
                                        if unsafe { (*p).spec.e_text } as i32 == 6 {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\u%04x".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        } else {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\%03o".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        }
                                        break '__s20;
                                    }
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    break '__s13;
                }
                {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
            }
            6 => {
                {
                    let mut z: *const u8 = z_txt_1 as *const u8;
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    while unsafe { *z } != 0 {
                        let mut i: u32 = 0 as u32;
                        {
                            i = 0 as u32;
                            '__b19: loop {
                                if !(unsafe { *z.add(i as usize) } as i32 >= 32 &&
                                                    unsafe { *z.add(i as usize) } as i32 != '\\' as i32 &&
                                                unsafe { *z.add(i as usize) } as i32 != '\"' as i32) {
                                    break '__b19;
                                }
                                '__c19: loop { break '__c19; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if i > 0 as u32 {
                            unsafe {
                                sqlite3_str_append(p_out_1, z as *const i8, i as i32)
                            };
                        }
                        if unsafe { *z.add(i as usize) } as i32 == 0 { break; }
                        '__s20:
                            {
                            match unsafe { *z.add(i as usize) } {
                                34 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\"".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                92 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\\\".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                8 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\b".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                12 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\f".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                10 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\n".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                13 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\r".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                9 => {
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\\t".as_ptr() as *mut i8 as *const i8, 2)
                                    };
                                }
                                _ => {
                                    {
                                        if unsafe { (*p).spec.e_text } as i32 == 6 {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\u%04x".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        } else {
                                            unsafe {
                                                sqlite3_str_appendf(p_out_1,
                                                    c"\\%03o".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { *z.add(i as usize) } as i32)
                                            };
                                        }
                                        break '__s20;
                                    }
                                }
                            }
                        }
                        {
                            let __n = i + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                    }
                    unsafe {
                        sqlite3_str_append(p_out_1,
                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    break '__s13;
                }
                {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
            }
            _ => {
                {
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt_1) };
                    break '__s13;
                }
            }
        }
    }
    if unsafe { (*p).spec.e_esc } as i32 != 1 {
        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, i_start);
    }
}

///* Transfer any error in pStr over into p.
extern "C" fn qrf_str_err(p: *mut Qrf, p_str_1: *mut Sqlite3Str) -> () {
    let rc: i32 =
        if !(p_str_1).is_null() {
            unsafe { sqlite3_str_errcode(p_str_1) }
        } else { 0 };
    if rc != 0 {
        unsafe {
            qrf_error(unsafe { &mut *p }, rc, unsafe { sqlite3_errstr(rc) })
        };
    }
}

///* Check to see if z[] is a valid VT100 escape.  If it is, then
///* return the number of bytes in the escape sequence.  Return 0 if
///* z[] is not a VT100 escape.
///*
///* This routine assumes that z[0] is \033 (ESC).
extern "C" fn qrf_is_vt100(z: *const u8) -> i32 {
    let mut i: i32 = 0;
    if unsafe { *z.offset(1 as isize) } as i32 != '[' as i32 { return 0; }
    i = 2;
    while unsafe { *z.offset(i as isize) } as i32 >= 48 &&
            unsafe { *z.offset(i as isize) } as i32 <= 63 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }
    while unsafe { *z.offset(i as isize) } as i32 >= 32 &&
            unsafe { *z.offset(i as isize) } as i32 <= 47 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }
    if (unsafe { *z.offset(i as isize) } as i32) < 64 ||
            unsafe { *z.offset(i as isize) } as i32 > 126 {
        return 0;
    }
    return i + 1;
}

///* Compute the value and length of a multi-byte UTF-8 character that
///* begins at z[0]. Return the length.  Write the Unicode value into *pU.
///*
///* This routine only works for *multi-byte* UTF-8 characters.  It does
///* not attempt to detect illegal characters.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_qrf_decode_utf8(z: *const u8, p_u_1: &mut i32)
    -> i32 {
    if unsafe { *z.offset(0 as isize) } as i32 & 224 == 192 &&
            unsafe { *z.offset(1 as isize) } as i32 & 192 == 128 {
        *p_u_1 =
            (unsafe { *z.offset(0 as isize) } as i32 & 31) << 6 |
                unsafe { *z.offset(1 as isize) } as i32 & 63;
        return 2;
    }
    if unsafe { *z.offset(0 as isize) } as i32 & 240 == 224 &&
                unsafe { *z.offset(1 as isize) } as i32 & 192 == 128 &&
            unsafe { *z.offset(2 as isize) } as i32 & 192 == 128 {
        *p_u_1 =
            (unsafe { *z.offset(0 as isize) } as i32 & 15) << 12 |
                    (unsafe { *z.offset(1 as isize) } as i32 & 63) << 6 |
                unsafe { *z.offset(2 as isize) } as i32 & 63;
        return 3;
    }
    if unsafe { *z.offset(0 as isize) } as i32 & 248 == 240 &&
                    unsafe { *z.offset(1 as isize) } as i32 & 192 == 128 &&
                unsafe { *z.offset(2 as isize) } as i32 & 192 == 128 &&
            unsafe { *z.offset(3 as isize) } as i32 & 192 == 128 {
        *p_u_1 =
            (unsafe { *z.offset(0 as isize) } as i32 & 15) << 18 |
                        (unsafe { *z.offset(1 as isize) } as i32 & 63) << 12 |
                    (unsafe { *z.offset(2 as isize) } as i32 & 63) << 6 |
                unsafe { *z.offset(3 as isize) } as i32 & 63;
        return 4;
    }
    *p_u_1 = 0;
    return 1;
}

/// Lookup table to estimate the number of columns consumed by a Unicode
///* character.
#[repr(C)]
#[derive(Copy, Clone)]
struct AnonS0 {
    w: u8,
    i_first: i32,
}

static a_qrf_u_width: [AnonS0; 303] =
    [AnonS0 { w: 0 as u8, i_first: 768 }, AnonS0 { w: 1 as u8, i_first: 880 },
            AnonS0 { w: 0 as u8, i_first: 1155 },
            AnonS0 { w: 1 as u8, i_first: 1159 },
            AnonS0 { w: 0 as u8, i_first: 1160 },
            AnonS0 { w: 1 as u8, i_first: 1162 },
            AnonS0 { w: 0 as u8, i_first: 1425 },
            AnonS0 { w: 1 as u8, i_first: 1470 },
            AnonS0 { w: 0 as u8, i_first: 1471 },
            AnonS0 { w: 1 as u8, i_first: 1472 },
            AnonS0 { w: 0 as u8, i_first: 1473 },
            AnonS0 { w: 1 as u8, i_first: 1475 },
            AnonS0 { w: 0 as u8, i_first: 1476 },
            AnonS0 { w: 1 as u8, i_first: 1478 },
            AnonS0 { w: 0 as u8, i_first: 1479 },
            AnonS0 { w: 1 as u8, i_first: 1480 },
            AnonS0 { w: 0 as u8, i_first: 1536 },
            AnonS0 { w: 1 as u8, i_first: 1540 },
            AnonS0 { w: 0 as u8, i_first: 1552 },
            AnonS0 { w: 1 as u8, i_first: 1558 },
            AnonS0 { w: 0 as u8, i_first: 1611 },
            AnonS0 { w: 1 as u8, i_first: 1631 },
            AnonS0 { w: 0 as u8, i_first: 1648 },
            AnonS0 { w: 1 as u8, i_first: 1649 },
            AnonS0 { w: 0 as u8, i_first: 1750 },
            AnonS0 { w: 1 as u8, i_first: 1765 },
            AnonS0 { w: 0 as u8, i_first: 1767 },
            AnonS0 { w: 1 as u8, i_first: 1769 },
            AnonS0 { w: 0 as u8, i_first: 1770 },
            AnonS0 { w: 1 as u8, i_first: 1774 },
            AnonS0 { w: 0 as u8, i_first: 1807 },
            AnonS0 { w: 1 as u8, i_first: 1808 },
            AnonS0 { w: 0 as u8, i_first: 1809 },
            AnonS0 { w: 1 as u8, i_first: 1810 },
            AnonS0 { w: 0 as u8, i_first: 1840 },
            AnonS0 { w: 1 as u8, i_first: 1867 },
            AnonS0 { w: 0 as u8, i_first: 1958 },
            AnonS0 { w: 1 as u8, i_first: 1969 },
            AnonS0 { w: 0 as u8, i_first: 2027 },
            AnonS0 { w: 1 as u8, i_first: 2036 },
            AnonS0 { w: 0 as u8, i_first: 2305 },
            AnonS0 { w: 1 as u8, i_first: 2307 },
            AnonS0 { w: 0 as u8, i_first: 2364 },
            AnonS0 { w: 1 as u8, i_first: 2365 },
            AnonS0 { w: 0 as u8, i_first: 2369 },
            AnonS0 { w: 1 as u8, i_first: 2377 },
            AnonS0 { w: 0 as u8, i_first: 2381 },
            AnonS0 { w: 1 as u8, i_first: 2382 },
            AnonS0 { w: 0 as u8, i_first: 2385 },
            AnonS0 { w: 1 as u8, i_first: 2389 },
            AnonS0 { w: 0 as u8, i_first: 2402 },
            AnonS0 { w: 1 as u8, i_first: 2404 },
            AnonS0 { w: 0 as u8, i_first: 2433 },
            AnonS0 { w: 1 as u8, i_first: 2434 },
            AnonS0 { w: 0 as u8, i_first: 2492 },
            AnonS0 { w: 1 as u8, i_first: 2493 },
            AnonS0 { w: 0 as u8, i_first: 2497 },
            AnonS0 { w: 1 as u8, i_first: 2501 },
            AnonS0 { w: 0 as u8, i_first: 2509 },
            AnonS0 { w: 1 as u8, i_first: 2510 },
            AnonS0 { w: 0 as u8, i_first: 2530 },
            AnonS0 { w: 1 as u8, i_first: 2532 },
            AnonS0 { w: 0 as u8, i_first: 2561 },
            AnonS0 { w: 1 as u8, i_first: 2563 },
            AnonS0 { w: 0 as u8, i_first: 2620 },
            AnonS0 { w: 1 as u8, i_first: 2621 },
            AnonS0 { w: 0 as u8, i_first: 2625 },
            AnonS0 { w: 1 as u8, i_first: 2627 },
            AnonS0 { w: 0 as u8, i_first: 2631 },
            AnonS0 { w: 1 as u8, i_first: 2633 },
            AnonS0 { w: 0 as u8, i_first: 2635 },
            AnonS0 { w: 1 as u8, i_first: 2638 },
            AnonS0 { w: 0 as u8, i_first: 2672 },
            AnonS0 { w: 1 as u8, i_first: 2674 },
            AnonS0 { w: 0 as u8, i_first: 2689 },
            AnonS0 { w: 1 as u8, i_first: 2691 },
            AnonS0 { w: 0 as u8, i_first: 2748 },
            AnonS0 { w: 1 as u8, i_first: 2749 },
            AnonS0 { w: 0 as u8, i_first: 2753 },
            AnonS0 { w: 1 as u8, i_first: 2758 },
            AnonS0 { w: 0 as u8, i_first: 2759 },
            AnonS0 { w: 1 as u8, i_first: 2761 },
            AnonS0 { w: 0 as u8, i_first: 2765 },
            AnonS0 { w: 1 as u8, i_first: 2766 },
            AnonS0 { w: 0 as u8, i_first: 2786 },
            AnonS0 { w: 1 as u8, i_first: 2788 },
            AnonS0 { w: 0 as u8, i_first: 2817 },
            AnonS0 { w: 1 as u8, i_first: 2818 },
            AnonS0 { w: 0 as u8, i_first: 2876 },
            AnonS0 { w: 1 as u8, i_first: 2877 },
            AnonS0 { w: 0 as u8, i_first: 2879 },
            AnonS0 { w: 1 as u8, i_first: 2880 },
            AnonS0 { w: 0 as u8, i_first: 2881 },
            AnonS0 { w: 1 as u8, i_first: 2884 },
            AnonS0 { w: 0 as u8, i_first: 2893 },
            AnonS0 { w: 1 as u8, i_first: 2894 },
            AnonS0 { w: 0 as u8, i_first: 2902 },
            AnonS0 { w: 1 as u8, i_first: 2903 },
            AnonS0 { w: 0 as u8, i_first: 2946 },
            AnonS0 { w: 1 as u8, i_first: 2947 },
            AnonS0 { w: 0 as u8, i_first: 3008 },
            AnonS0 { w: 1 as u8, i_first: 3009 },
            AnonS0 { w: 0 as u8, i_first: 3021 },
            AnonS0 { w: 1 as u8, i_first: 3022 },
            AnonS0 { w: 0 as u8, i_first: 3134 },
            AnonS0 { w: 1 as u8, i_first: 3137 },
            AnonS0 { w: 0 as u8, i_first: 3142 },
            AnonS0 { w: 1 as u8, i_first: 3145 },
            AnonS0 { w: 0 as u8, i_first: 3146 },
            AnonS0 { w: 1 as u8, i_first: 3150 },
            AnonS0 { w: 0 as u8, i_first: 3157 },
            AnonS0 { w: 1 as u8, i_first: 3159 },
            AnonS0 { w: 0 as u8, i_first: 3260 },
            AnonS0 { w: 1 as u8, i_first: 3261 },
            AnonS0 { w: 0 as u8, i_first: 3263 },
            AnonS0 { w: 1 as u8, i_first: 3264 },
            AnonS0 { w: 0 as u8, i_first: 3270 },
            AnonS0 { w: 1 as u8, i_first: 3271 },
            AnonS0 { w: 0 as u8, i_first: 3276 },
            AnonS0 { w: 1 as u8, i_first: 3278 },
            AnonS0 { w: 0 as u8, i_first: 3298 },
            AnonS0 { w: 1 as u8, i_first: 3300 },
            AnonS0 { w: 0 as u8, i_first: 3393 },
            AnonS0 { w: 1 as u8, i_first: 3396 },
            AnonS0 { w: 0 as u8, i_first: 3405 },
            AnonS0 { w: 1 as u8, i_first: 3406 },
            AnonS0 { w: 0 as u8, i_first: 3530 },
            AnonS0 { w: 1 as u8, i_first: 3531 },
            AnonS0 { w: 0 as u8, i_first: 3538 },
            AnonS0 { w: 1 as u8, i_first: 3541 },
            AnonS0 { w: 0 as u8, i_first: 3542 },
            AnonS0 { w: 1 as u8, i_first: 3543 },
            AnonS0 { w: 0 as u8, i_first: 3633 },
            AnonS0 { w: 1 as u8, i_first: 3634 },
            AnonS0 { w: 0 as u8, i_first: 3636 },
            AnonS0 { w: 1 as u8, i_first: 3643 },
            AnonS0 { w: 0 as u8, i_first: 3655 },
            AnonS0 { w: 1 as u8, i_first: 3663 },
            AnonS0 { w: 0 as u8, i_first: 3761 },
            AnonS0 { w: 1 as u8, i_first: 3762 },
            AnonS0 { w: 0 as u8, i_first: 3764 },
            AnonS0 { w: 1 as u8, i_first: 3770 },
            AnonS0 { w: 0 as u8, i_first: 3771 },
            AnonS0 { w: 1 as u8, i_first: 3773 },
            AnonS0 { w: 0 as u8, i_first: 3784 },
            AnonS0 { w: 1 as u8, i_first: 3790 },
            AnonS0 { w: 0 as u8, i_first: 3864 },
            AnonS0 { w: 1 as u8, i_first: 3866 },
            AnonS0 { w: 0 as u8, i_first: 3893 },
            AnonS0 { w: 1 as u8, i_first: 3894 },
            AnonS0 { w: 0 as u8, i_first: 3895 },
            AnonS0 { w: 1 as u8, i_first: 3896 },
            AnonS0 { w: 0 as u8, i_first: 3897 },
            AnonS0 { w: 1 as u8, i_first: 3898 },
            AnonS0 { w: 0 as u8, i_first: 3953 },
            AnonS0 { w: 1 as u8, i_first: 3967 },
            AnonS0 { w: 0 as u8, i_first: 3968 },
            AnonS0 { w: 1 as u8, i_first: 3973 },
            AnonS0 { w: 0 as u8, i_first: 3974 },
            AnonS0 { w: 1 as u8, i_first: 3976 },
            AnonS0 { w: 0 as u8, i_first: 3984 },
            AnonS0 { w: 1 as u8, i_first: 3992 },
            AnonS0 { w: 0 as u8, i_first: 3993 },
            AnonS0 { w: 1 as u8, i_first: 4029 },
            AnonS0 { w: 0 as u8, i_first: 4038 },
            AnonS0 { w: 1 as u8, i_first: 4039 },
            AnonS0 { w: 0 as u8, i_first: 4141 },
            AnonS0 { w: 1 as u8, i_first: 4145 },
            AnonS0 { w: 0 as u8, i_first: 4146 },
            AnonS0 { w: 1 as u8, i_first: 4147 },
            AnonS0 { w: 0 as u8, i_first: 4150 },
            AnonS0 { w: 1 as u8, i_first: 4155 },
            AnonS0 { w: 0 as u8, i_first: 4184 },
            AnonS0 { w: 1 as u8, i_first: 4186 },
            AnonS0 { w: 2 as u8, i_first: 4352 },
            AnonS0 { w: 0 as u8, i_first: 4448 },
            AnonS0 { w: 1 as u8, i_first: 4608 },
            AnonS0 { w: 0 as u8, i_first: 4959 },
            AnonS0 { w: 1 as u8, i_first: 4960 },
            AnonS0 { w: 0 as u8, i_first: 5906 },
            AnonS0 { w: 1 as u8, i_first: 5909 },
            AnonS0 { w: 0 as u8, i_first: 5938 },
            AnonS0 { w: 1 as u8, i_first: 5941 },
            AnonS0 { w: 0 as u8, i_first: 5970 },
            AnonS0 { w: 1 as u8, i_first: 5972 },
            AnonS0 { w: 0 as u8, i_first: 6002 },
            AnonS0 { w: 1 as u8, i_first: 6004 },
            AnonS0 { w: 0 as u8, i_first: 6068 },
            AnonS0 { w: 1 as u8, i_first: 6070 },
            AnonS0 { w: 0 as u8, i_first: 6071 },
            AnonS0 { w: 1 as u8, i_first: 6078 },
            AnonS0 { w: 0 as u8, i_first: 6086 },
            AnonS0 { w: 1 as u8, i_first: 6087 },
            AnonS0 { w: 0 as u8, i_first: 6089 },
            AnonS0 { w: 1 as u8, i_first: 6100 },
            AnonS0 { w: 0 as u8, i_first: 6109 },
            AnonS0 { w: 1 as u8, i_first: 6110 },
            AnonS0 { w: 0 as u8, i_first: 6155 },
            AnonS0 { w: 1 as u8, i_first: 6158 },
            AnonS0 { w: 0 as u8, i_first: 6313 },
            AnonS0 { w: 1 as u8, i_first: 6314 },
            AnonS0 { w: 0 as u8, i_first: 6432 },
            AnonS0 { w: 1 as u8, i_first: 6435 },
            AnonS0 { w: 0 as u8, i_first: 6439 },
            AnonS0 { w: 1 as u8, i_first: 6441 },
            AnonS0 { w: 0 as u8, i_first: 6450 },
            AnonS0 { w: 1 as u8, i_first: 6451 },
            AnonS0 { w: 0 as u8, i_first: 6457 },
            AnonS0 { w: 1 as u8, i_first: 6460 },
            AnonS0 { w: 0 as u8, i_first: 6679 },
            AnonS0 { w: 1 as u8, i_first: 6681 },
            AnonS0 { w: 0 as u8, i_first: 6912 },
            AnonS0 { w: 1 as u8, i_first: 6916 },
            AnonS0 { w: 0 as u8, i_first: 6964 },
            AnonS0 { w: 1 as u8, i_first: 6965 },
            AnonS0 { w: 0 as u8, i_first: 6966 },
            AnonS0 { w: 1 as u8, i_first: 6971 },
            AnonS0 { w: 0 as u8, i_first: 6972 },
            AnonS0 { w: 1 as u8, i_first: 6973 },
            AnonS0 { w: 0 as u8, i_first: 6978 },
            AnonS0 { w: 1 as u8, i_first: 6979 },
            AnonS0 { w: 0 as u8, i_first: 7019 },
            AnonS0 { w: 1 as u8, i_first: 7028 },
            AnonS0 { w: 0 as u8, i_first: 7616 },
            AnonS0 { w: 1 as u8, i_first: 7627 },
            AnonS0 { w: 0 as u8, i_first: 7678 },
            AnonS0 { w: 1 as u8, i_first: 7680 },
            AnonS0 { w: 0 as u8, i_first: 8203 },
            AnonS0 { w: 1 as u8, i_first: 8208 },
            AnonS0 { w: 0 as u8, i_first: 8234 },
            AnonS0 { w: 1 as u8, i_first: 8239 },
            AnonS0 { w: 0 as u8, i_first: 8288 },
            AnonS0 { w: 1 as u8, i_first: 8292 },
            AnonS0 { w: 0 as u8, i_first: 8298 },
            AnonS0 { w: 1 as u8, i_first: 8304 },
            AnonS0 { w: 0 as u8, i_first: 8400 },
            AnonS0 { w: 1 as u8, i_first: 8432 },
            AnonS0 { w: 2 as u8, i_first: 9001 },
            AnonS0 { w: 1 as u8, i_first: 9003 },
            AnonS0 { w: 2 as u8, i_first: 11904 },
            AnonS0 { w: 0 as u8, i_first: 12330 },
            AnonS0 { w: 2 as u8, i_first: 12336 },
            AnonS0 { w: 1 as u8, i_first: 12351 },
            AnonS0 { w: 2 as u8, i_first: 12352 },
            AnonS0 { w: 0 as u8, i_first: 12441 },
            AnonS0 { w: 2 as u8, i_first: 12443 },
            AnonS0 { w: 1 as u8, i_first: 42192 },
            AnonS0 { w: 0 as u8, i_first: 43014 },
            AnonS0 { w: 1 as u8, i_first: 43015 },
            AnonS0 { w: 0 as u8, i_first: 43019 },
            AnonS0 { w: 1 as u8, i_first: 43020 },
            AnonS0 { w: 0 as u8, i_first: 43045 },
            AnonS0 { w: 1 as u8, i_first: 43047 },
            AnonS0 { w: 2 as u8, i_first: 44032 },
            AnonS0 { w: 1 as u8, i_first: 55204 },
            AnonS0 { w: 2 as u8, i_first: 63744 },
            AnonS0 { w: 1 as u8, i_first: 64256 },
            AnonS0 { w: 0 as u8, i_first: 64286 },
            AnonS0 { w: 1 as u8, i_first: 64287 },
            AnonS0 { w: 0 as u8, i_first: 65024 },
            AnonS0 { w: 2 as u8, i_first: 65040 },
            AnonS0 { w: 1 as u8, i_first: 65050 },
            AnonS0 { w: 0 as u8, i_first: 65056 },
            AnonS0 { w: 1 as u8, i_first: 65060 },
            AnonS0 { w: 2 as u8, i_first: 65072 },
            AnonS0 { w: 1 as u8, i_first: 65136 },
            AnonS0 { w: 0 as u8, i_first: 65279 },
            AnonS0 { w: 2 as u8, i_first: 65280 },
            AnonS0 { w: 1 as u8, i_first: 65377 },
            AnonS0 { w: 2 as u8, i_first: 65504 },
            AnonS0 { w: 1 as u8, i_first: 65511 },
            AnonS0 { w: 0 as u8, i_first: 65529 },
            AnonS0 { w: 1 as u8, i_first: 65532 },
            AnonS0 { w: 0 as u8, i_first: 68097 },
            AnonS0 { w: 1 as u8, i_first: 68100 },
            AnonS0 { w: 0 as u8, i_first: 68101 },
            AnonS0 { w: 1 as u8, i_first: 68103 },
            AnonS0 { w: 0 as u8, i_first: 68108 },
            AnonS0 { w: 1 as u8, i_first: 68112 },
            AnonS0 { w: 0 as u8, i_first: 68152 },
            AnonS0 { w: 1 as u8, i_first: 68155 },
            AnonS0 { w: 0 as u8, i_first: 68159 },
            AnonS0 { w: 1 as u8, i_first: 68160 },
            AnonS0 { w: 0 as u8, i_first: 119143 },
            AnonS0 { w: 1 as u8, i_first: 119146 },
            AnonS0 { w: 0 as u8, i_first: 119155 },
            AnonS0 { w: 1 as u8, i_first: 119171 },
            AnonS0 { w: 0 as u8, i_first: 119173 },
            AnonS0 { w: 1 as u8, i_first: 119180 },
            AnonS0 { w: 0 as u8, i_first: 119210 },
            AnonS0 { w: 1 as u8, i_first: 119214 },
            AnonS0 { w: 0 as u8, i_first: 119362 },
            AnonS0 { w: 1 as u8, i_first: 119365 },
            AnonS0 { w: 2 as u8, i_first: 131072 },
            AnonS0 { w: 1 as u8, i_first: 196606 },
            AnonS0 { w: 2 as u8, i_first: 196608 },
            AnonS0 { w: 1 as u8, i_first: 262142 },
            AnonS0 { w: 0 as u8, i_first: 917505 },
            AnonS0 { w: 1 as u8, i_first: 917506 },
            AnonS0 { w: 0 as u8, i_first: 917536 },
            AnonS0 { w: 1 as u8, i_first: 917632 },
            AnonS0 { w: 0 as u8, i_first: 917760 },
            AnonS0 { w: 1 as u8, i_first: 918000 }];

///* Auxiliary routines contined within this module that might be useful
///* in other contexts, and which are therefore exported.
////
////*
///* Return an estimate of the width, in columns, for the single Unicode
///* character c.  For normal characters, the answer is always 1.  But the
///* estimate might be 0 or 2 for zero-width and double-width characters.
///*
///* Different devices display unicode using different widths.  So
///* it is impossible to know that true display width with 100% accuracy.
///* Inaccuracies in the width estimates might cause columns to be misaligned.
///* Unfortunately, there is nothing we can do about that.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_qrf_wcwidth(c: i32) -> i32 {
    let mut i_first: i32 = 0;
    let mut i_last: i32 = 0;
    if c < 768 { return 1; }

    /// The general case
    (i_first = 0);
    i_last =
        (core::mem::size_of::<[AnonS0; 303]>() as u64 / 8 - 1 as u64) as i32;
    while i_first < i_last - 1 {
        let i_mid: i32 = (i_first + i_last) / 2;
        let c_mid: i32 = a_qrf_u_width[i_mid as usize].i_first as i32;
        if c_mid < c {
            i_first = i_mid;
        } else if c_mid > c {
            i_last = i_mid - 1;
        } else { return a_qrf_u_width[i_mid as usize].w as i32; }
    }
    if a_qrf_u_width[i_last as usize].i_first as i32 > c {
        return a_qrf_u_width[i_first as usize].w as i32;
    }
    return a_qrf_u_width[i_last as usize].w as i32;
}

///* Adjust the input string zIn[] such that it is no more than N display
///* characters wide.  If it is wider than that, then truncate and add
///* ellipsis.  Or if zIn[] contains a \r or \n, truncate at that point,
///* adding ellipsis.  Embedded tabs in zIn[] are converted into ordinary
///* spaces.
///*
///* Return this display width of the modified title string.
extern "C" fn qrf_title_limit(z_in_1: *mut i8, n_1: i32) -> i32 {
    let mut z: *mut u8 = z_in_1 as *mut u8;
    let mut n: i32 = 0;
    let mut z_ellipsis: *mut u8 = core::ptr::null_mut();
    loop {
        if (unsafe { *z.offset(0 as isize) } as i32) < ' ' as i32 {
            let mut k: i32 = 0;
            if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                z_ellipsis = core::ptr::null_mut();
                break;
            } else if unsafe { *z.offset(0 as isize) } as i32 ==
                        '\u{1b}' as i32 &&
                    { k = qrf_is_vt100(z as *const u8); k } > 0 {
                {
                    let __n = k;
                    let __p = &mut z;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            } else if unsafe { *z.offset(0 as isize) } as i32 == '\t' as i32 {
                unsafe { *z.offset(0 as isize) = ' ' as i32 as u8 };
            } else if unsafe { *z.offset(0 as isize) } as i32 == '\n' as i32
                    || unsafe { *z.offset(0 as isize) } as i32 == '\r' as i32 {
                unsafe { *z.offset(0 as isize) = ' ' as i32 as u8 };
            } else {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        } else if 128 & unsafe { *z.offset(0 as isize) } as i32 == 0 {
            if n >= n_1 - 3 && z_ellipsis == core::ptr::null_mut() {
                z_ellipsis = z;
            }
            if n == n_1 { unsafe { *z.offset(0 as isize) = 0 as u8 }; break; }
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else {
            let mut u: i32 = 0;
            let len: i32 = sqlite3_qrf_decode_utf8(z as *const u8, &mut u);
            if n + len > n_1 - 3 && z_ellipsis == core::ptr::null_mut() {
                z_ellipsis = z;
            }
            if n + len > n_1 {
                unsafe { *z.offset(0 as isize) = 0 as u8 };
                break;
            }
            {
                let __n = len;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += sqlite3_qrf_wcwidth(u);
        }
    }
    if !(z_ellipsis).is_null() && n_1 >= 3 {
        unsafe {
            memcpy(z_ellipsis as *mut (),
                c"...".as_ptr() as *mut i8 as *const (), 4 as u64)
        };
    }
    return n;
}

///* Return the display width of the longest line of text
///* in the (possibly) multi-line input string zIn[0..nByte].
///* zIn[] is not necessarily zero-terminated.  Take
///* into account tab characters, zero- and double-width
///* characters, CR and NL, and VT100 escape codes.
///*
///* Write the number of newlines into *pnNL.  So, *pnNL will
///* return 0 if everything fits on one line, or positive it
///* it will need to be split.
extern "C" fn qrf_display_width(mut z_in_1: *const i8, n_byte_1: Sqlite3Int64,
    pn_nl_1: *mut i32) -> i32 {
    let mut z: *const u8 = core::ptr::null();
    let mut z_end: *const u8 = core::ptr::null();
    let mut mx: i32 = 0;
    let mut n: i32 = 0;
    let mut n_nl: i32 = 0;
    if z_in_1 == core::ptr::null() {
        z_in_1 = c"".as_ptr() as *mut i8 as *const i8;
    }
    z = z_in_1 as *const u8;
    z_end = unsafe { z.offset(n_byte_1 as isize) };
    while z < z_end {
        if (unsafe { *z.offset(0 as isize) } as i32) < ' ' as i32 {
            let mut k: i32 = 0;
            if unsafe { *z.offset(0 as isize) } as i32 == '\u{1b}' as i32 &&
                    { k = qrf_is_vt100(z); k } > 0 {
                {
                    let __n = k;
                    let __p = &mut z;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            } else {
                if unsafe { *z.offset(0 as isize) } as i32 == '\t' as i32 {
                    n = n + 8 & !7;
                } else if unsafe { *z.offset(0 as isize) } as i32 ==
                            '\n' as i32 ||
                        unsafe { *z.offset(0 as isize) } as i32 == '\r' as i32 {
                    { let __p = &mut n_nl; let __t = *__p; *__p += 1; __t };
                    if n > mx { mx = n; }
                    n = 0;
                }
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        } else if 128 & unsafe { *z.offset(0 as isize) } as i32 == 0 {
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else {
            let mut u: i32 = 0;
            let len: i32 = sqlite3_qrf_decode_utf8(z, &mut u);
            {
                let __n = len;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += sqlite3_qrf_wcwidth(u);
        }
    }
    if mx > n { n = mx; }
    if !(pn_nl_1).is_null() { unsafe { *pn_nl_1 = n_nl }; }
    return n;
}

///* Do a quick sanity check to see aBlob[0..nBlob-1] is valid JSONB
///* return true if it is and false if it is not.
///*
///* False positives are possible, but not false negatives.
#[allow(unused_doc_comments)]
extern "C" fn qrf_jsonb_quick_check(a_blob_1: *const u8, n_blob_1: i32)
    -> i32 {
    let mut x: u8 = 0 as u8;
    /// Payload size half-byte
    let mut i: i32 = 0;
    /// Loop counter
    let mut n: i32 = 0;
    /// Bytes in the payload size integer
    let mut sz: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    if n_blob_1 == 0 { return 0; }
    x = (unsafe { *a_blob_1.offset(0 as isize) } as i32 >> 4) as u8;
    if x as i32 <= 11 { return (n_blob_1 == 1 + x as i32) as i32; }
    n = if (x as i32) < 14 { x as i32 - 11 } else { 4 * (x as i32 - 13) };
    if n_blob_1 < 1 + n { return 0; }
    sz = unsafe { *a_blob_1.offset(1 as isize) } as Sqlite3Uint64;
    {
        i = 1;
        '__b26: loop {
            if !(i < n) { break '__b26; }
            '__c26: loop {
                sz =
                    (sz << 8) +
                        unsafe { *a_blob_1.offset((i + 1) as isize) } as
                            Sqlite3Uint64;
                break '__c26;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return (sz + n as Sqlite3Uint64 + 1 as Sqlite3Uint64 ==
                n_blob_1 as Sqlite3Uint64) as i32;
}

///* The current iCol-th column of p->pStmt is known to be a BLOB.  Check
///* to see if that BLOB is really a JSONB blob.  If it is, then translate
///* it into a text JSON representation and return a pointer to that text JSON.
///* If the BLOB is not JSONB, then return a NULL pointer.
///*
///* The memory used to hold the JSON text is managed internally by the
///* "p" object and is overwritten and/or deallocated upon the next call
///* to this routine (with the same p argument) or when the p object is
///* finailized.
extern "C" fn qrf_jsonb_to_json(p: &mut Qrf, i_col_1: i32) -> *const i8 {
    let mut n_byte: i32 = 0;
    let mut p_blob: *const () = core::ptr::null();
    let mut rc: i32 = 0;
    n_byte = unsafe { sqlite3_column_bytes((*p).p_stmt, i_col_1) };
    p_blob = unsafe { sqlite3_column_blob((*p).p_stmt, i_col_1) };
    if qrf_jsonb_quick_check(p_blob as *mut u8 as *const u8, n_byte) == 0 {
        return core::ptr::null();
    }
    if (*p).p_j_trans == core::ptr::null_mut() {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                    &mut db)
            };
        if rc != 0 { unsafe { sqlite3_close(db) }; return core::ptr::null(); }
        rc =
            unsafe {
                sqlite3_prepare_v2(db,
                    c"SELECT json(?1)".as_ptr() as *mut i8 as *const i8, -1,
                    &mut (*p).p_j_trans, core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe { sqlite3_finalize((*p).p_j_trans) };
            (*p).p_j_trans = core::ptr::null_mut();
            unsafe { sqlite3_close(db) };
            return core::ptr::null();
        }
    } else { unsafe { sqlite3_reset((*p).p_j_trans) }; }
    unsafe {
        sqlite3_bind_blob((*p).p_j_trans, 1, p_blob as *mut () as *const (),
            n_byte, None)
    };
    rc = unsafe { sqlite3_step((*p).p_j_trans) };
    if rc == 100 {
        return unsafe { sqlite3_column_text((*p).p_j_trans, 0) } as *const i8;
    } else { return core::ptr::null(); }
}

///* If xWrite is defined, send all content of pOut to xWrite and
///* reset pOut.
extern "C" fn qrf_write(p: *mut Qrf) -> () {
    let mut n: i32 = 0;
    if unsafe { (*p).spec.x_write.is_some() } &&
            { n = unsafe { sqlite3_str_length(unsafe { (*p).p_out }) }; n } >
                0 {
        let rc: i32 =
            unsafe {
                (unsafe {
                        (*p).spec.x_write.unwrap()
                    })(unsafe { (*p).spec.p_write_arg },
                    unsafe { sqlite3_str_value(unsafe { (*p).p_out }) } as
                        *const i8, n as Sqlite3Int64)
            };
        unsafe { sqlite3_str_reset(unsafe { (*p).p_out }) };
        if rc != 0 {
            unsafe {
                qrf_error(unsafe { &mut *p }, rc,
                    c"Failed to write %d bytes of output".as_ptr() as *mut i8 as
                        *const i8, n)
            };
        }
    }
}

///* Render value pVal into pOut
extern "C" fn qrf_render_value(p: *mut Qrf, p_out_1: *mut Sqlite3Str,
    i_col_1: i32) -> () {
    let i_start_len: i32 = unsafe { sqlite3_str_length(p_out_1) };
    if unsafe { (*p).spec.x_render.is_some() } {
        let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
        let mut z: *mut i8 = core::ptr::null_mut();
        p_val =
            unsafe {
                sqlite3_value_dup(unsafe {
                            sqlite3_column_value(unsafe { (*p).p_stmt }, i_col_1)
                        } as *const Sqlite3Value)
            };
        z =
            unsafe {
                (unsafe {
                        (*p).spec.x_render.unwrap()
                    })(unsafe { (*p).spec.p_render_arg }, p_val)
            };
        unsafe { sqlite3_value_free(p_val) };
        if !(z).is_null() {
            unsafe { sqlite3_str_appendall(p_out_1, z as *const i8) };
            unsafe { sqlite3_free(z as *mut ()) };
            return;
        }
    }
    '__s27:
        {
        match unsafe { sqlite3_column_type(unsafe { (*p).p_stmt }, i_col_1) }
            {
            1 => {
                {
                    unsafe {
                        sqlite3_str_appendf(p_out_1,
                            c"%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                sqlite3_column_int64(unsafe { (*p).p_stmt }, i_col_1)
                            })
                    };
                    break '__s27;
                }
                {
                    let z_txt: *const i8 =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p).p_stmt }, i_col_1)
                            } as *const i8;
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt) };
                    break '__s27;
                }
                {
                    if unsafe { (*p).spec.b_text_jsonb } as i32 == 2 {
                        let z_json: *const i8 =
                            qrf_jsonb_to_json(unsafe { &mut *p }, i_col_1);
                        if !(z_json).is_null() {
                            if unsafe { (*p).spec.e_text } as i32 == 2 {
                                unsafe {
                                    sqlite3_str_append(p_out_1,
                                        c"jsonb(".as_ptr() as *mut i8 as *const i8, 6)
                                };
                                qrf_encode_text(p, p_out_1, z_json);
                                unsafe {
                                    sqlite3_str_append(p_out_1,
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            } else { qrf_encode_text(p, p_out_1, z_json); }
                            break '__s27;
                        }
                    }
                    '__s28:
                        {
                        match unsafe { (*p).spec.e_blob } {
                            3 => {
                                {
                                    let mut i_start: i32 = 0;
                                    let n_blob: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_append(p_out_1,
                                                c"x\'".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    i_start = unsafe { sqlite3_str_length(p_out_1) };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_appendchar(p_out_1, 1, '\'' as i32 as i8)
                                        };
                                    }
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start };
                                        '__b29: loop {
                                            if !(i < n_blob) { break '__b29; }
                                            '__c29: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe {
                                                    *z_val.offset(j as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                        }
                                                };
                                                unsafe {
                                                    *z_val.offset((j + 1) as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 & 15) as isize)
                                                        }
                                                };
                                                break '__c29;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += 2
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            2 => {
                                {
                                    let mut i_start: i32 = 0;
                                    let n_blob: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_append(p_out_1,
                                                c"x\'".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    i_start = unsafe { sqlite3_str_length(p_out_1) };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_appendchar(p_out_1, 1, '\'' as i32 as i8)
                                        };
                                    }
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start };
                                        '__b29: loop {
                                            if !(i < n_blob) { break '__b29; }
                                            '__c29: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe {
                                                    *z_val.offset(j as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                        }
                                                };
                                                unsafe {
                                                    *z_val.offset((j + 1) as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 & 15) as isize)
                                                        }
                                                };
                                                break '__c29;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += 2
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            4 => {
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            5 => {
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            6 => {
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            _ => {
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                        }
                    }
                    break '__s27;
                }
                {
                    unsafe {
                        sqlite3_str_appendall(p_out_1,
                            unsafe { (*p).spec.z_null } as *const i8)
                    };
                    break '__s27;
                }
                {
                    let z_txt_1: *const i8 =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p).p_stmt }, i_col_1)
                            } as *const i8;
                    qrf_encode_text(p, p_out_1, z_txt_1);
                    break '__s27;
                }
            }
            2 => {
                {
                    let z_txt: *const i8 =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p).p_stmt }, i_col_1)
                            } as *const i8;
                    unsafe { sqlite3_str_appendall(p_out_1, z_txt) };
                    break '__s27;
                }
                {
                    if unsafe { (*p).spec.b_text_jsonb } as i32 == 2 {
                        let z_json: *const i8 =
                            qrf_jsonb_to_json(unsafe { &mut *p }, i_col_1);
                        if !(z_json).is_null() {
                            if unsafe { (*p).spec.e_text } as i32 == 2 {
                                unsafe {
                                    sqlite3_str_append(p_out_1,
                                        c"jsonb(".as_ptr() as *mut i8 as *const i8, 6)
                                };
                                qrf_encode_text(p, p_out_1, z_json);
                                unsafe {
                                    sqlite3_str_append(p_out_1,
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            } else { qrf_encode_text(p, p_out_1, z_json); }
                            break '__s27;
                        }
                    }
                    '__s28:
                        {
                        match unsafe { (*p).spec.e_blob } {
                            3 => {
                                {
                                    let mut i_start: i32 = 0;
                                    let n_blob: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_append(p_out_1,
                                                c"x\'".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    i_start = unsafe { sqlite3_str_length(p_out_1) };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_appendchar(p_out_1, 1, '\'' as i32 as i8)
                                        };
                                    }
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start };
                                        '__b29: loop {
                                            if !(i < n_blob) { break '__b29; }
                                            '__c29: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe {
                                                    *z_val.offset(j as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                        }
                                                };
                                                unsafe {
                                                    *z_val.offset((j + 1) as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 & 15) as isize)
                                                        }
                                                };
                                                break '__c29;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += 2
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            2 => {
                                {
                                    let mut i_start: i32 = 0;
                                    let n_blob: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_append(p_out_1,
                                                c"x\'".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    i_start = unsafe { sqlite3_str_length(p_out_1) };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_appendchar(p_out_1, 1, '\'' as i32 as i8)
                                        };
                                    }
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start };
                                        '__b29: loop {
                                            if !(i < n_blob) { break '__b29; }
                                            '__c29: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe {
                                                    *z_val.offset(j as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                        }
                                                };
                                                unsafe {
                                                    *z_val.offset((j + 1) as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 & 15) as isize)
                                                        }
                                                };
                                                break '__c29;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += 2
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            4 => {
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            5 => {
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            6 => {
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            _ => {
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                        }
                    }
                    break '__s27;
                }
                {
                    unsafe {
                        sqlite3_str_appendall(p_out_1,
                            unsafe { (*p).spec.z_null } as *const i8)
                    };
                    break '__s27;
                }
                {
                    let z_txt_1: *const i8 =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p).p_stmt }, i_col_1)
                            } as *const i8;
                    qrf_encode_text(p, p_out_1, z_txt_1);
                    break '__s27;
                }
            }
            4 => {
                {
                    if unsafe { (*p).spec.b_text_jsonb } as i32 == 2 {
                        let z_json: *const i8 =
                            qrf_jsonb_to_json(unsafe { &mut *p }, i_col_1);
                        if !(z_json).is_null() {
                            if unsafe { (*p).spec.e_text } as i32 == 2 {
                                unsafe {
                                    sqlite3_str_append(p_out_1,
                                        c"jsonb(".as_ptr() as *mut i8 as *const i8, 6)
                                };
                                qrf_encode_text(p, p_out_1, z_json);
                                unsafe {
                                    sqlite3_str_append(p_out_1,
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            } else { qrf_encode_text(p, p_out_1, z_json); }
                            break '__s27;
                        }
                    }
                    '__s28:
                        {
                        match unsafe { (*p).spec.e_blob } {
                            3 => {
                                {
                                    let mut i_start: i32 = 0;
                                    let n_blob: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_append(p_out_1,
                                                c"x\'".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    i_start = unsafe { sqlite3_str_length(p_out_1) };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_appendchar(p_out_1, 1, '\'' as i32 as i8)
                                        };
                                    }
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start };
                                        '__b29: loop {
                                            if !(i < n_blob) { break '__b29; }
                                            '__c29: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe {
                                                    *z_val.offset(j as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                        }
                                                };
                                                unsafe {
                                                    *z_val.offset((j + 1) as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 & 15) as isize)
                                                        }
                                                };
                                                break '__c29;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += 2
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            2 => {
                                {
                                    let mut i_start: i32 = 0;
                                    let n_blob: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_append(p_out_1,
                                                c"x\'".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    i_start = unsafe { sqlite3_str_length(p_out_1) };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, n_blob, ' ' as i32 as i8)
                                    };
                                    if unsafe { (*p).spec.e_blob } as i32 == 2 {
                                        unsafe {
                                            sqlite3_str_appendchar(p_out_1, 1, '\'' as i32 as i8)
                                        };
                                    }
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start };
                                        '__b29: loop {
                                            if !(i < n_blob) { break '__b29; }
                                            '__c29: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe {
                                                    *z_val.offset(j as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                        }
                                                };
                                                unsafe {
                                                    *z_val.offset((j + 1) as isize) =
                                                        unsafe {
                                                            *(c"0123456789abcdef".as_ptr() as
                                                                        *mut i8).offset((c as i32 & 15) as isize)
                                                        }
                                                };
                                                break '__c29;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += 2
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            4 => {
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            5 => {
                                {
                                    let mut i_start_1: i32 = 0;
                                    let n_blob_1: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut i: i32 = 0;
                                    let mut j: i32 = 0;
                                    let mut z_val_1: *mut i8 = core::ptr::null_mut();
                                    let a: *const u8 =
                                        unsafe {
                                                sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                            } as *const u8;
                                    let sz_c: i32 =
                                        if unsafe { (*p).spec.e_blob } as i32 == 5 { 6 } else { 4 };
                                    unsafe {
                                        sqlite3_str_append(p_out_1,
                                            c"\"".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                    i_start_1 = unsafe { sqlite3_str_length(p_out_1) };
                                    {
                                        i = sz_c;
                                        '__b30: loop {
                                            if !(i > 0) { break '__b30; }
                                            '__c30: loop {
                                                unsafe {
                                                    sqlite3_str_appendchar(p_out_1, n_blob_1, ' ' as i32 as i8)
                                                };
                                                break '__c30;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                        }
                                    }
                                    unsafe {
                                        sqlite3_str_appendchar(p_out_1, 1, '\"' as i32 as i8)
                                    };
                                    if unsafe { sqlite3_str_errcode(p_out_1) } != 0 { return; }
                                    z_val_1 = unsafe { sqlite3_str_value(p_out_1) };
                                    {
                                        { i = 0; j = i_start_1 };
                                        '__b31: loop {
                                            if !(i < n_blob_1) { break '__b31; }
                                            '__c31: loop {
                                                let c: u8 = unsafe { *a.offset(i as isize) } as u8;
                                                unsafe { *z_val_1.offset(j as isize) = '\\' as i32 as i8 };
                                                if sz_c == 4 {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) =
                                                            ('0' as i32 + (c as i32 >> 6 & 3)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) =
                                                            ('0' as i32 + (c as i32 >> 3 & 7)) as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) =
                                                            ('0' as i32 + (c as i32 & 7)) as i8
                                                    };
                                                } else {
                                                    unsafe {
                                                        *z_val_1.offset((j + 1) as isize) = 'u' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 2) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 3) as isize) = '0' as i32 as i8
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 4) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 >> 4 & 15) as isize)
                                                            }
                                                    };
                                                    unsafe {
                                                        *z_val_1.offset((j + 5) as isize) =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((c as i32 & 15) as isize)
                                                            }
                                                    };
                                                }
                                                break '__c31;
                                            }
                                            {
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                j += sz_c
                                            };
                                        }
                                    }
                                    break '__s28;
                                }
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            6 => {
                                {
                                    let n_blob_2: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    unsafe {
                                        sqlite3_str_appendf(p_out_1,
                                            c"(%d-byte blob)".as_ptr() as *mut i8 as *const i8,
                                            n_blob_2)
                                    };
                                    break '__s28;
                                }
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                            _ => {
                                {
                                    let p_blob: *const () =
                                        unsafe {
                                            sqlite3_column_blob(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let n_blob_3: i32 =
                                        unsafe {
                                            sqlite3_column_bytes(unsafe { (*p).p_stmt }, i_col_1)
                                        };
                                    let mut rc: i32 = 0;
                                    qrf_write(p);
                                    if n_blob_3 == 0
                                        {} else if unsafe { (*p).spec.e_esc } as i32 == 1 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p).spec.x_write.unwrap()
                                                    })(unsafe { (*p).spec.p_write_arg }, p_blob as *const i8,
                                                    n_blob_3 as i64)
                                            };
                                        if rc != 0 {
                                            unsafe {
                                                qrf_error(unsafe { &mut *p }, rc,
                                                    c"Failed to write %d bytes of BLOB output".as_ptr() as
                                                            *mut i8 as *const i8, n_blob_3)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_out_1, p_blob as *const i8, n_blob_3)
                                        };
                                        qrf_escape(unsafe { (*p).spec.e_esc } as i32, p_out_1, 0);
                                        qrf_write(p);
                                    }
                                }
                            }
                        }
                    }
                    break '__s27;
                }
                {
                    unsafe {
                        sqlite3_str_appendall(p_out_1,
                            unsafe { (*p).spec.z_null } as *const i8)
                    };
                    break '__s27;
                }
                {
                    let z_txt_1: *const i8 =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p).p_stmt }, i_col_1)
                            } as *const i8;
                    qrf_encode_text(p, p_out_1, z_txt_1);
                    break '__s27;
                }
            }
            5 => {
                {
                    unsafe {
                        sqlite3_str_appendall(p_out_1,
                            unsafe { (*p).spec.z_null } as *const i8)
                    };
                    break '__s27;
                }
                {
                    let z_txt_1: *const i8 =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p).p_stmt }, i_col_1)
                            } as *const i8;
                    qrf_encode_text(p, p_out_1, z_txt_1);
                    break '__s27;
                }
            }
            3 => {
                {
                    let z_txt_1: *const i8 =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p).p_stmt }, i_col_1)
                            } as *const i8;
                    qrf_encode_text(p, p_out_1, z_txt_1);
                    break '__s27;
                }
            }
            _ => {}
        }
    }
    if unsafe { (*p).spec.n_char_limit } > 0 &&
            unsafe { sqlite3_str_length(p_out_1) } - i_start_len >
                unsafe { (*p).spec.n_char_limit } {
        let mut z: *const u8 = core::ptr::null();
        let mut ii: i32 = 0;
        let mut w: i32 = 0;
        let mut limit: i32 = unsafe { (*p).spec.n_char_limit };
        z =
            unsafe {
                (unsafe { sqlite3_str_value(p_out_1) } as
                        *const u8).offset(i_start_len as isize)
            };
        if limit < 4 { limit = 4; }
        loop {
            if (unsafe { *z.offset(ii as isize) } as i32) < ' ' as i32 {
                let mut k: i32 = 0;
                if unsafe { *z.offset(ii as isize) } as i32 == '\u{1b}' as i32
                        &&
                        { k = qrf_is_vt100(unsafe { z.offset(ii as isize) }); k } >
                            0 {
                    ii += k;
                } else if unsafe { *z.offset(ii as isize) } as i32 == 0 {
                    break;
                } else {
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            } else if 128 & unsafe { *z.offset(ii as isize) } as i32 == 0 {
                { let __p = &mut w; let __t = *__p; *__p += 1; __t };
                if w > limit { break; }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            } else {
                let mut u: i32 = 0;
                let len: i32 =
                    sqlite3_qrf_decode_utf8(unsafe { &*z.offset(ii as isize) },
                        &mut u);
                w += sqlite3_qrf_wcwidth(u);
                if w > limit { break; }
                ii += len;
            }
        }
        if w > limit {
            unsafe { sqlite3_str_truncate(p_out_1, i_start_len + ii) };
            unsafe {
                sqlite3_str_append(p_out_1,
                    c"...".as_ptr() as *mut i8 as *const i8, 3)
            };
        }
    }
}

///* Load into pData the default alignment for the body of a table.
extern "C" fn qrf_load_alignment(p_data_1: &QrfColData, p: &Qrf) -> () {
    let mut i: Sqlite3Int64 = 0 as Sqlite3Int64;
    {
        i = 0 as Sqlite3Int64;
        '__b33: loop {
            if !(i < (*p_data_1).n_col as i64) { break '__b33; }
            '__c33: loop {
                unsafe {
                    (*(*p_data_1).a.offset(i as isize)).e =
                        (*p).spec.e_dflt_align
                };
                if i < (*p).spec.n_align as i64 {
                    let ax: u8 =
                        unsafe { *(*p).spec.a_align.offset(i as isize) };
                    if ax as i32 & 3 != 0 {
                        unsafe {
                            (*(*p_data_1).a.offset(i as isize)).e =
                                (ax as i32 & 3 |
                                        unsafe { (*(*p_data_1).a.offset(i as isize)).e } as i32 &
                                            12) as u8
                        };
                    }
                } else if i < (*p).spec.n_width as i64 {
                    if (unsafe { *(*p).spec.a_width.offset(i as isize) } as i32)
                            < 0 {
                        unsafe {
                            (*(*p_data_1).a.offset(i as isize)).e =
                                (3 |
                                        unsafe { (*(*p_data_1).a.offset(i as isize)).e } as i32 &
                                            12) as u8
                        };
                    }
                }
                break '__c33;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* If the single column in pData->a[] with pData->n entries can be
///* laid out as nCol columns with a 2-space gap between each such
///* that all columns fit within nSW, then return a pointer to an array
///* of integers which is the width of each column from left to right.
///*
///* If the layout is not possible, return a NULL pointer.
///*
///* Space to hold the returned array is from sqlite_malloc64().
#[allow(unused_doc_comments)]
extern "C" fn qrf_valid_layout(p_data_1: &QrfColData, p: *mut Qrf,
    n_col_1: i32, n_sw_1: i32) -> *mut i32 {
    let mut i: i32 = 0;
    /// Loop counter
    let mut nr: i32 = 0;
    /// Number of rows
    let mut w: i32 = 0;
    /// Width of the current column
    let mut t: i64 = 0 as i64;
    /// Total width of all columns
    let mut aw: *mut i32 = core::ptr::null_mut();

    /// Array of individual column widths
    (aw =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<i32>() as u64 *
                        n_col_1 as u64)
            } as *mut i32);
    if aw == core::ptr::null_mut() {
        qrf_oom(p);
        return core::ptr::null_mut();
    }
    nr =
        (((*p_data_1).n + n_col_1 as Sqlite3Int64 - 1 as Sqlite3Int64) /
                n_col_1 as Sqlite3Int64) as i32;
    {
        i = 0;
        '__b34: loop {
            if !((i as Sqlite3Int64) < (*p_data_1).n) { break '__b34; }
            '__c34: loop {
                if i % nr == 0 {
                    if i > 0 {
                        unsafe { *aw.offset((i / nr - 1) as isize) = w };
                    }
                    w = unsafe { *(*p_data_1).ai_wth.offset(i as isize) };
                } else if unsafe { *(*p_data_1).ai_wth.offset(i as isize) } >
                        w {
                    w = unsafe { *(*p_data_1).ai_wth.offset(i as isize) };
                }
                break '__c34;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *aw.offset((n_col_1 - 1) as isize) = w };
    {
        t = { i = 0; i } as i64;
        '__b35: loop {
            if !(i < n_col_1) { break '__b35; }
            '__c35: loop {
                t += unsafe { *aw.offset(i as isize) } as i64;
                break '__c35;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    t += (2 * (n_col_1 - 1)) as i64;
    if t > n_sw_1 as i64 {
        unsafe { sqlite3_free(aw as *mut ()) };
        return core::ptr::null_mut();
    }
    return aw;
}

///* The output is single-column and the bSplitColumn flag is set.
///* Check to see if the single-column output can be split into multiple
///* columns that appear side-by-side.  Adjust pData appropriately.
extern "C" fn qrf_split_column(p_data_1: *mut QrfColData, p: *mut Qrf) -> () {
    let mut n_col: i32 = 1;
    let mut aw: *mut i32 = core::ptr::null_mut();
    let mut az: *mut *mut i8 = core::ptr::null_mut();
    let mut ai_wth: *mut i32 = core::ptr::null_mut();
    let mut ab_num: *mut u8 = core::ptr::null_mut();
    let mut n_col_next: i32 = 2;
    let mut w: i32 = 0;
    let mut a: *mut QrfPerCol = core::ptr::null_mut();
    let mut n_row: Sqlite3Int64 = 1 as Sqlite3Int64;
    let mut i: Sqlite3Int64 = 0 as Sqlite3Int64;
    loop {
        let aw_new: *mut i32 =
            qrf_valid_layout(unsafe { &*p_data_1 }, p, n_col_next,
                unsafe { (*p).spec.n_screen_width } as i32);
        if aw_new == core::ptr::null_mut() { break; }
        unsafe { sqlite3_free(aw as *mut ()) };
        aw = aw_new;
        n_col = n_col_next;
        n_row =
            (unsafe { (*p_data_1).n } + n_col as Sqlite3Int64 -
                    1 as Sqlite3Int64) / n_col as Sqlite3Int64;
        if n_row == 1 as i64 { break; }
        { let __p = &mut n_col_next; let __t = *__p; *__p += 1; __t };
        while (unsafe { (*p_data_1).n } + n_col_next as Sqlite3Int64 -
                        1 as Sqlite3Int64) / n_col_next as Sqlite3Int64 == n_row {
            { let __p = &mut n_col_next; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_col == 1 { unsafe { sqlite3_free(aw as *mut ()) }; return; }
    az =
        unsafe {
                sqlite3_malloc64((n_row * n_col as Sqlite3Int64) as u64 *
                        core::mem::size_of::<*mut i8>() as u64)
            } as *mut *mut i8;
    if az == core::ptr::null_mut() { qrf_oom(p); return; }
    ai_wth =
        unsafe {
                sqlite3_malloc64((n_row * n_col as Sqlite3Int64) as u64 *
                        core::mem::size_of::<i32>() as u64)
            } as *mut i32;
    if ai_wth == core::ptr::null_mut() {
        unsafe { sqlite3_free(az as *mut ()) };
        qrf_oom(p);
        return;
    }
    a =
        unsafe {
                sqlite3_malloc64(n_col as u64 *
                        core::mem::size_of::<QrfPerCol>() as u64)
            } as *mut QrfPerCol;
    if a == core::ptr::null_mut() {
        unsafe { sqlite3_free(az as *mut ()) };
        unsafe { sqlite3_free(ai_wth as *mut ()) };
        qrf_oom(p);
        return;
    }
    ab_num =
        unsafe {
                sqlite3_malloc64((n_row * n_col as Sqlite3Int64) as
                        Sqlite3Uint64)
            } as *mut u8;
    if ab_num == core::ptr::null_mut() {
        unsafe { sqlite3_free(az as *mut ()) };
        unsafe { sqlite3_free(ai_wth as *mut ()) };
        unsafe { sqlite3_free(a as *mut ()) };
        qrf_oom(p);
        return;
    }
    {
        i = 0 as Sqlite3Int64;
        '__b38: loop {
            if !(i < unsafe { (*p_data_1).n }) { break '__b38; }
            '__c38: loop {
                let j: Sqlite3Int64 =
                    i % n_row * n_col as Sqlite3Int64 + i / n_row;
                unsafe {
                    *az.offset(j as isize) =
                        unsafe { *unsafe { (*p_data_1).az.offset(i as isize) } }
                };
                unsafe {
                    *ab_num.offset(j as isize) =
                        unsafe { *unsafe { (*p_data_1).ab_num.offset(i as isize) } }
                };
                unsafe {
                    *unsafe { (*p_data_1).az.offset(i as isize) } =
                        core::ptr::null_mut()
                };
                unsafe {
                    *ai_wth.offset(j as isize) =
                        unsafe { *unsafe { (*p_data_1).ai_wth.offset(i as isize) } }
                };
                break '__c38;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    while i < n_row * n_col as Sqlite3Int64 {
        let j: Sqlite3Int64 = i % n_row * n_col as Sqlite3Int64 + i / n_row;
        unsafe {
            *az.offset(j as isize) =
                unsafe {
                    sqlite3_mprintf(c"".as_ptr() as *mut i8 as *const i8)
                }
        };
        if unsafe { *az.offset(j as isize) } == core::ptr::null_mut() {
            qrf_oom(p);
        }
        unsafe { *ai_wth.offset(j as isize) = 0 };
        unsafe { *ab_num.offset(j as isize) = 0 as u8 };
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }
    {
        i = 0 as Sqlite3Int64;
        '__b40: loop {
            if !(i < n_col as i64) { break '__b40; }
            '__c40: loop {
                unsafe {
                    (*a.offset(i as isize)).fx =
                        {
                                unsafe {
                                    (*a.offset(i as isize)).mx_w =
                                        {
                                            unsafe {
                                                (*a.offset(i as isize)).w =
                                                    unsafe { *aw.offset(i as isize) }
                                            };
                                            unsafe { (*a.offset(i as isize)).w }
                                        }
                                };
                                unsafe { (*a.offset(i as isize)).mx_w }
                            } as u8
                };
                unsafe {
                    (*a.offset(i as isize)).e =
                        unsafe { (*unsafe { (*p_data_1).a.offset(0 as isize) }).e }
                };
                break '__c40;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p_data_1).az } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p_data_1).ai_wth } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p_data_1).a } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p_data_1).ab_num } as *mut ()) };
    unsafe { sqlite3_free(aw as *mut ()) };
    unsafe { (*p_data_1).az = az };
    unsafe { (*p_data_1).ai_wth = ai_wth };
    unsafe { (*p_data_1).a = a };
    unsafe { (*p_data_1).ab_num = ab_num };
    unsafe { (*p_data_1).n_col = n_col };
    unsafe {
        (*p_data_1).n =
            {
                unsafe {
                    (*p_data_1).n_alloc = n_row * n_col as Sqlite3Int64
                };
                unsafe { (*p_data_1).n_alloc }
            }
    };
    {
        i = { w = 0; w } as Sqlite3Int64;
        '__b41: loop {
            if !(i < n_col as i64) { break '__b41; }
            '__c41: loop {
                w += unsafe { (*a.offset(i as isize)).w };
                break '__c41;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        (*p_data_1).n_margin =
            ((unsafe { (*p).spec.n_screen_width } as i32 - w) / (n_col - 1))
                as u8
    };
    if unsafe { (*p_data_1).n_margin } as i32 > 5 {
        unsafe { (*p_data_1).n_margin = 5 as u8 };
    }
}

///* Adjust the layout for the screen width restriction
#[allow(unused_doc_comments)]
extern "C" fn qrf_restrict_screen_width(p_data_1: &mut QrfColData, p: &Qrf)
    -> () {
    let mut sep_w: i32 = 0;
    /// Width of all box separators and margins
    let mut sum_w: i32 = 0;
    /// Total width of data area over all columns
    let mut target_w: i32 = 0;
    /// Desired total data area
    let mut i: i32 = 0;
    /// Loop counters
    let mut n_col: i32 = 0;

    /// Number of columns
    ((*p_data_1).n_margin = 2 as u8);
    if (*p).spec.n_screen_width as i32 == 0 { return; }
    if (*p).spec.e_style as i32 == 2 {
        sep_w = (*p_data_1).n_col * 2 - 2;
    } else {
        sep_w = (*p_data_1).n_col * 3 + 1;
        if (*p).spec.b_border as i32 == 1 { sep_w -= 2; }
    }
    n_col = (*p_data_1).n_col;
    {
        { i = 0; sum_w = 0 };
        '__b42: loop {
            if !(i < n_col) { break '__b42; }
            '__c42: loop {
                if sum_w >
                        2147483647 -
                            unsafe { (*(*p_data_1).a.offset(i as isize)).w } {
                    return;
                }
                sum_w += unsafe { (*(*p_data_1).a.offset(i as isize)).w };
                break '__c42;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if (*p).spec.n_screen_width as i64 >= sum_w as i64 + sep_w as i64 {
        return;
    }

    /// First thing to do is reduce the separation between columns
    ((*p_data_1).n_margin = 0 as u8);
    if (*p).spec.e_style as i32 == 2 {
        sep_w = (*p_data_1).n_col - 1;
    } else {
        sep_w = (*p_data_1).n_col + 1;
        if (*p).spec.b_border as i32 == 1 { sep_w -= 2; }
    }
    target_w = (*p).spec.n_screen_width as i32 - sep_w;
    while sum_w > target_w {
        let mut gain: i32 = 0;
        let mut w: i32 = 0;
        let mut ix: i32 = -1;
        let mut mx: i32 = 0;
        {
            i = 0;
            '__b44: loop {
                if !(i < n_col) { break '__b44; }
                '__c44: loop {
                    if unsafe { (*(*p_data_1).a.offset(i as isize)).fx } as i32
                                        == 0 &&
                                    { w = unsafe { (*(*p_data_1).a.offset(i as isize)).w }; w }
                                        > mx && w > 8 &&
                            (w > 16 ||
                                w * 2 > unsafe { (*(*p_data_1).a.offset(i as isize)).mx_w })
                        {
                        ix = i;
                        mx = w;
                    }
                    break '__c44;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if ix < 0 { break; }
        if mx >= 8 * 2 { gain = mx / 2; } else { gain = mx - 8; }
        if sum_w - gain < target_w { gain = sum_w - target_w; }
        sum_w -= gain;
        unsafe { (*(*p_data_1).a.offset(ix as isize)).w -= gain };
        (*p_data_1).b_multi_row = 1 as u8;
    }
}

/// Draw horizontal line N characters long using unicode box
///* characters
#[allow(unused_doc_comments)]
extern "C" fn qrf_box_line(p_out_1: *mut Sqlite3Str, n_1: i32, b_dbl_1: i32)
    -> () {
    let az_dash: [*const i8; 2] =
        [c"\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}".as_ptr()
                    as *const i8,
                c"\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}".as_ptr()
                    as *const i8];
    ///  0       1      2     3      4        5      6      7      8      9
    let n_dash: i32 = 30 as i32;
    let mut nn: i64 = 3 as i64 * n_1 as i64;
    while nn > n_dash as i64 {
        unsafe {
            sqlite3_str_append(p_out_1, az_dash[b_dbl_1 as usize], n_dash)
        };
        nn -= n_dash as i64;
    }
    unsafe {
        sqlite3_str_append(p_out_1, az_dash[b_dbl_1 as usize], nn as i32)
    };
}

///* Draw a horizontal separator for a QRF_STYLE_Box table.
extern "C" fn qrf_box_separator(p_out_1: *mut Sqlite3Str, p: &QrfColData,
    z_sep1_1: *const i8, z_sep2_1: *const i8, z_sep3_1: *const i8,
    b_dbl_1: i32) -> () {
    let mut i: i32 = 0;
    if (*p).n_col > 0 {
        let use_border: i32 =
            (unsafe { (*(*p).p).spec.b_border } as i32 != 1) as i32;
        if use_border != 0 {
            unsafe { sqlite3_str_appendall(p_out_1, z_sep1_1) };
        }
        qrf_box_line(p_out_1,
            unsafe { (*(*p).a.offset(0 as isize)).w } + (*p).n_margin as i32,
            b_dbl_1);
        {
            i = 1;
            '__b46: loop {
                if !(i < (*p).n_col) { break '__b46; }
                '__c46: loop {
                    unsafe { sqlite3_str_appendall(p_out_1, z_sep2_1) };
                    qrf_box_line(p_out_1,
                        unsafe { (*(*p).a.offset(i as isize)).w } +
                            (*p).n_margin as i32, b_dbl_1);
                    break '__c46;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if use_border != 0 {
            unsafe { sqlite3_str_appendall(p_out_1, z_sep3_1) };
        }
    }
    unsafe {
        sqlite3_str_append(p_out_1, c"\n".as_ptr() as *mut i8 as *const i8, 1)
    };
}

///* Print a markdown or table-style row separator using ascii-art
extern "C" fn qrf_row_separator(p_out_1: *mut Sqlite3Str, p: &QrfColData,
    mut c_sep_1: i8) -> () {
    let mut i: i32 = 0;
    if (*p).n_col > 0 {
        let use_border: i32 =
            (unsafe { (*(*p).p).spec.b_border } as i32 != 1) as i32;
        if use_border != 0 {
            unsafe {
                sqlite3_str_append(p_out_1, &raw mut c_sep_1 as *const i8, 1)
            };
        }
        unsafe {
            sqlite3_str_appendchar(p_out_1,
                unsafe { (*(*p).a.offset(0 as isize)).w } +
                    (*p).n_margin as i32, '-' as i32 as i8)
        };
        {
            i = 1;
            '__b47: loop {
                if !(i < (*p).n_col) { break '__b47; }
                '__c47: loop {
                    unsafe {
                        sqlite3_str_append(p_out_1, &raw mut c_sep_1 as *const i8,
                            1)
                    };
                    unsafe {
                        sqlite3_str_appendchar(p_out_1,
                            unsafe { (*(*p).a.offset(i as isize)).w } +
                                (*p).n_margin as i32, '-' as i32 as i8)
                    };
                    break '__c47;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if use_border != 0 {
            unsafe {
                sqlite3_str_append(p_out_1, &raw mut c_sep_1 as *const i8, 1)
            };
        }
    }
    unsafe {
        sqlite3_str_append(p_out_1, c"\n".as_ptr() as *mut i8 as *const i8, 1)
    };
}

///* (*pz)[] is a line of text that is to be displayed the box or table or
///* similar tabular formats.  z[] contain newlines or might be too wide
///* to fit in the columns so will need to be split into multiple line.
///*
///* This routine determines:
///*
///*    *  How many bytes of z[] should be shown on the current line.
///*    *  How many character positions those bytes will cover.
///*    *  The byte offset to the start of the next line.
#[allow(unused_doc_comments)]
extern "C" fn qrf_wrap_line(z_in_1: *const i8, w: i32, b_wrap_1: i32,
    pn_this_1: &mut i32, pn_wide_1: &mut i32, pi_next_1: &mut i32) -> () {
    let mut i: i32 = 0;
    /// Input bytes consumed
    let mut k: i32 = 0;
    /// Bytes in a VT100 code
    let mut n: i32 = 0;
    /// Output column number
    let z: *const u8 = z_in_1 as *const u8;
    let mut c: u8 = 0 as u8;
    if unsafe { *z.offset(0 as isize) } as i32 == 0 {
        *pn_this_1 = 0;
        *pn_wide_1 = 0;
        *pi_next_1 = 0;
        return;
    }
    n = 0;
    {
        i = 0;
        '__b48: loop {
            if !(n <= w) { break '__b48; }
            '__c48: loop {
                c = unsafe { *z.offset(i as isize) } as u8;
                if c as i32 >= 192 {
                    let mut u: i32 = 0;
                    let len: i32 =
                        sqlite3_qrf_decode_utf8(unsafe { &*z.offset(i as isize) },
                            &mut u);
                    let wcw: i32 = sqlite3_qrf_wcwidth(u);
                    if wcw + n > w { break '__b48; }
                    i += len - 1;
                    n += wcw;
                    break '__c48;
                }
                if c as i32 >= ' ' as i32 {
                    if n == w { break '__b48; }
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                    break '__c48;
                }
                if c as i32 == 0 || c as i32 == '\n' as i32 { break '__b48; }
                if c as i32 == '\r' as i32 &&
                        unsafe { *z.offset((i + 1) as isize) } as i32 == '\n' as i32
                    {
                    c =
                        unsafe {
                                *z.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                            } as u8;
                    break '__b48;
                }
                if c as i32 == '\t' as i32 {
                    let wcw: i32 = 8 - (n & 7);
                    if n + wcw > w { break '__b48; }
                    n += wcw;
                    break '__c48;
                }
                if c as i32 == 27 &&
                        { k = qrf_is_vt100(unsafe { &*z.offset(i as isize) }); k } >
                            0 {
                    i += k - 1;
                } else if n == w {
                    break '__b48;
                } else {
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                }
                break '__c48;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if c as i32 == 0 {
        *pn_this_1 = i;
        *pn_wide_1 = n;
        *pi_next_1 = i;
        return;
    }
    if c as i32 == '\n' as i32 {
        *pn_this_1 = i;
        *pn_wide_1 = n;
        *pi_next_1 = i + 1;
        return;
    }
    if b_wrap_1 != 0 && unsafe { *z.offset(i as isize) } as i32 != 0 &&
                !(qrf_c_type[unsafe { *z.offset(i as isize) } as u8 as usize]
                                        as i32 & 1 != 0) as i32 != 0 &&
            (qrf_c_type[c as u8 as usize] as i32 & 6 != 0) as i32 ==
                (qrf_c_type[unsafe { *z.offset(i as isize) } as u8 as usize]
                                as i32 & 6 != 0) as i32 {
        {
            k = i - 1;
            '__b49: loop {
                if !(k >= i / 2) { break '__b49; }
                '__c49: loop {
                    if qrf_c_type[unsafe { *z.offset(k as isize) } as u8 as
                                            usize] as i32 & 1 != 0 {
                        break '__b49;
                    }
                    break '__c49;
                }
                { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
            }
        }
        if k < i / 2 && i / 2 > 0 {
            {
                k = i;
                '__b50: loop {
                    if !(k >= i / 2) { break '__b50; }
                    '__c50: loop {
                        if (qrf_c_type[unsafe { *z.offset((k - 1) as isize) } as u8
                                                            as usize] as i32 & 6 != 0) as i32 !=
                                    (qrf_c_type[unsafe { *z.offset(k as isize) } as u8 as usize]
                                                    as i32 & 6 != 0) as i32 &&
                                unsafe { *z.offset(k as isize) } as i32 & 192 != 128 {
                            break '__b50;
                        }
                        break '__c50;
                    }
                    { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                }
            }
        }
        if k >= i / 2 {
            i = k;
            n =
                qrf_display_width(z as *const i8, k as Sqlite3Int64,
                    core::ptr::null_mut());
        }
    }
    *pn_this_1 = i;
    *pn_wide_1 = n;
    while unsafe { *z_in_1.offset(i as isize) } as i32 == ' ' as i32 ||
                unsafe { *z_in_1.offset(i as isize) } as i32 == '\t' as i32 ||
            unsafe { *z_in_1.offset(i as isize) } as i32 == '\r' as i32 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }
    *pi_next_1 = i;
}

///* Append nVal bytes of text from zVal onto the end of pOut.
///* Convert tab characters in zVal to the appropriate number of
///* spaces.
extern "C" fn qrf_append_with_tabs(p_out_1: *mut Sqlite3Str,
    z_val_1: *const i8, mut n_val_1: i32) -> () {
    let mut i: i32 = 0;
    let mut col: u32 = 0 as u32;
    let mut z: *mut u8 = z_val_1 as *mut u8;
    while i < n_val_1 {
        let c: u8 = unsafe { *z.offset(i as isize) };
        if (c as i32) < ' ' as i32 {
            let mut k: i32 = 0;
            unsafe { sqlite3_str_append(p_out_1, z as *const i8, i) };
            n_val_1 -= i;
            {
                let __n = i;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            i = 0;
            if c as i32 == '\u{1b}' as i32 &&
                    { k = qrf_is_vt100(z as *const u8); k } > 0 {
                unsafe { sqlite3_str_append(p_out_1, z as *const i8, k) };
                {
                    let __n = k;
                    let __p = &mut z;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                n_val_1 -= k;
            } else if c as i32 == '\t' as i32 {
                k = (8 as u32 - (col & 7 as u32)) as i32;
                unsafe {
                    sqlite3_str_appendchar(p_out_1, k, ' ' as i32 as i8)
                };
                col += k as u32;
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_val_1; let __t = *__p; *__p -= 1; __t };
            } else if c as i32 == '\r' as i32 && n_val_1 == 1 {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_val_1; let __t = *__p; *__p -= 1; __t };
            } else {
                let mut z_ctrl_pik: [i8; 4] = [0; 4];
                { let __p = &mut col; let __t = *__p; *__p += 1; __t };
                z_ctrl_pik[0 as usize] = 226u8 as i8;
                z_ctrl_pik[1 as usize] = 144u8 as i8;
                z_ctrl_pik[2 as usize] = (128 + c as i32) as i8;
                unsafe {
                    sqlite3_str_append(p_out_1,
                        &raw mut z_ctrl_pik[0 as usize] as *mut i8 as *const i8, 3)
                };
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_val_1; let __t = *__p; *__p -= 1; __t };
            }
        } else if 128 & c as i32 == 0 {
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            { let __p = &mut col; let __t = *__p; *__p += 1; __t };
        } else {
            let mut u: i32 = 0;
            let len: i32 =
                sqlite3_qrf_decode_utf8(unsafe {
                            &raw mut *z.offset(i as isize)
                        } as *const u8, &mut u);
            i += len;
            col += sqlite3_qrf_wcwidth(u) as u32;
        }
    }
    unsafe { sqlite3_str_append(p_out_1, z as *const i8, i) };
}

///* Output horizontally justified text into pOut.  The text is the
///* first nVal bytes of zVal.  Include nWS bytes of whitespace, either
///* split between both sides, or on the left, or on the right, depending
///* on eAlign.
#[allow(unused_doc_comments)]
extern "C" fn qrf_print_aligned(p_out_1: *mut Sqlite3Str, p_col_1: &QrfPerCol,
    n_val_1: i32, n_ws_1: i32) -> () {
    let mut e_align: u8 = ((*p_col_1).e as i32 & 3) as u8;
    if e_align as i32 == 0 && (*p_col_1).b_num != 0 { e_align = 3 as u8; }
    if e_align as i32 == 2 {

        /// Center the text
        unsafe {
            sqlite3_str_appendchar(p_out_1, n_ws_1 / 2, ' ' as i32 as i8)
        };
        qrf_append_with_tabs(p_out_1, (*p_col_1).z as *const i8, n_val_1);
        unsafe {
            sqlite3_str_appendchar(p_out_1, n_ws_1 - n_ws_1 / 2,
                ' ' as i32 as i8)
        };
    } else if e_align as i32 == 3 {

        /// Right justify the text
        unsafe { sqlite3_str_appendchar(p_out_1, n_ws_1, ' ' as i32 as i8) };
        qrf_append_with_tabs(p_out_1, (*p_col_1).z as *const i8, n_val_1);
    } else {

        /// Left justify the text
        qrf_append_with_tabs(p_out_1, (*p_col_1).z as *const i8, n_val_1);
        unsafe { sqlite3_str_appendchar(p_out_1, n_ws_1, ' ' as i32 as i8) };
    }
}

/// Trim spaces of the end if pOut
extern "C" fn qrf_r_trim(p_out_1: *mut Sqlite3Str) -> () {
    let mut n_byte: i32 = unsafe { sqlite3_str_length(p_out_1) };
    let z_out: *const i8 = unsafe { sqlite3_str_value(p_out_1) } as *const i8;
    while n_byte > 0 &&
            unsafe { *z_out.offset((n_byte - 1) as isize) } as i32 ==
                ' ' as i32 {
        { let __p = &mut n_byte; let __t = *__p; *__p -= 1; __t };
    }
    unsafe { sqlite3_str_truncate(p_out_1, n_byte) };
}

///* Columnar modes require that the entire query be evaluated first, with
///* results written into memory, so that we can compute appropriate column
///* widths.
#[allow(unused_doc_comments)]
extern "C" fn qrf_columnar(p: *mut Qrf) -> () {
    let mut i: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut j: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Loop counters
    let mut col_sep: *const i8 = core::ptr::null();
    /// Column separator text
    let mut row_sep: *const i8 = core::ptr::null();
    /// Row terminator text
    let mut row_start: *const i8 = core::ptr::null();
    /// Row start text
    let mut sz_col_sep: i32 = 0;
    let mut sz_row_sep: i32 = 0;
    let mut sz_row_start: i32 = 0;
    /// Size in bytes of previous 3
    let mut rc: i32 = 0;
    /// Result code
    let mut n_column: i32 = unsafe { (*p).n_col };
    /// Number of columns
    let mut b_ww: i32 = 0;
    /// True to do word-wrap
    let mut p_str: *mut Sqlite3Str = core::ptr::null_mut();
    /// Temporary rendering
    let mut data: QrfColData = unsafe { core::mem::zeroed() };
    /// Columnar layout data
    let mut b_r_trim: i32 = 0;

    /// Trim trailing space
    (rc = unsafe { sqlite3_step(unsafe { (*p).p_stmt }) });
    if rc != 100 || n_column == 0 { return; }

    /// Initialize the data container
    unsafe {
        memset(&raw mut data as *mut (), 0,
            core::mem::size_of::<QrfColData>() as u64)
    };
    data.n_col = unsafe { (*p).n_col };
    data.p = p;
    data.a =
        unsafe {
                sqlite3_malloc64(n_column as u64 *
                        core::mem::size_of::<QrfPerCol>() as u64)
            } as *mut QrfPerCol;
    if data.a == core::ptr::null_mut() { qrf_oom(p); return; }
    unsafe {
        memset(data.a as *mut (), 0,
            n_column as u64 * core::mem::size_of::<QrfPerCol>() as u64)
    };
    if qrf_col_data_enlarge(&mut data) != 0 { return; }
    if !(data.az != core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"qrfColumnar".as_ptr() as *const i8,
                c"qrf.c".as_ptr() as *mut i8 as *const i8, 1970,
                c"data.az!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p).spec.b_titles } as i32 == 2 {
        let saved_e_text: u8 = unsafe { (*p).spec.e_text };
        unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
        unsafe { memset(data.ab_num as *mut (), 0, n_column as u64) };
        {
            i = 0 as Sqlite3Int64;
            '__b54: loop {
                if !(i < n_column as i64) { break '__b54; }
                '__c54: loop {
                    let mut z: *const i8 =
                        unsafe {
                                sqlite3_column_name(unsafe { (*p).p_stmt }, i as i32)
                            } as *const i8;
                    let mut n_nl: i32 = 0;
                    let mut n: i32 = 0;
                    let mut w: i32 = 0;
                    p_str = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                    qrf_encode_text(p, p_str,
                        if !(z).is_null() {
                            z
                        } else { c"".as_ptr() as *mut i8 as *const i8 });
                    n = unsafe { sqlite3_str_length(p_str) };
                    qrf_str_err(p, p_str);
                    z =
                        {
                                unsafe {
                                    *data.az.offset(data.n as isize) =
                                        unsafe { sqlite3_str_finish(p_str) }
                                };
                                unsafe { *data.az.offset(data.n as isize) }
                            } as *const i8;
                    if unsafe { (*p).spec.n_title_limit } != 0 {
                        n_nl = 0;
                        unsafe {
                            *data.ai_wth.offset(data.n as isize) =
                                {
                                    w =
                                        qrf_title_limit(unsafe { *data.az.offset(data.n as isize) },
                                            unsafe { (*p).spec.n_title_limit } as i32);
                                    w
                                }
                        };
                    } else {
                        unsafe {
                            *data.ai_wth.offset(data.n as isize) =
                                {
                                    w = qrf_display_width(z, n as Sqlite3Int64, &mut n_nl);
                                    w
                                }
                        };
                    }
                    { let __p = &mut data.n; let __t = *__p; *__p += 1; __t };
                    if w > unsafe { (*data.a.offset(i as isize)).mx_w } {
                        unsafe { (*data.a.offset(i as isize)).mx_w = w };
                    }
                    if n_nl != 0 { data.b_multi_row = 1 as u8; }
                    break '__c54;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*p).spec.e_text = saved_e_text };
        {
            let __p = unsafe { &mut (*p).n_row };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    '__b55: loop {
        '__c55: loop {
            if data.n + n_column as Sqlite3Int64 > data.n_alloc {
                if qrf_col_data_enlarge(&mut data) != 0 { return; }
            }
            {
                i = 0 as Sqlite3Int64;
                '__b56: loop {
                    if !(i < n_column as i64) { break '__b56; }
                    '__c56: loop {
                        let mut z: *const i8 = core::ptr::null();
                        let mut n_nl_1: i32 = 0;
                        let mut n: i32 = 0;
                        let mut w: i32 = 0;
                        let e_type: i32 =
                            unsafe {
                                sqlite3_column_type(unsafe { (*p).p_stmt }, i as i32)
                            };
                        p_str = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        qrf_render_value(p, p_str, i as i32);
                        n = unsafe { sqlite3_str_length(p_str) };
                        qrf_str_err(p, p_str);
                        z =
                            {
                                unsafe {
                                    *data.az.offset(data.n as isize) =
                                        unsafe { sqlite3_str_finish(p_str) }
                                };
                                unsafe { *data.az.offset(data.n as isize) }
                            };
                        unsafe {
                            *data.ab_num.offset(data.n as isize) =
                                (e_type == 1 || e_type == 2) as u8
                        };
                        unsafe {
                            *data.ai_wth.offset(data.n as isize) =
                                {
                                    w =
                                        qrf_display_width(z as *const i8, n as Sqlite3Int64,
                                            &mut n_nl_1);
                                    w
                                }
                        };
                        { let __p = &mut data.n; let __t = *__p; *__p += 1; __t };
                        if w > unsafe { (*data.a.offset(i as isize)).mx_w } {
                            unsafe { (*data.a.offset(i as isize)).mx_w = w };
                        }
                        if n_nl_1 != 0 { data.b_multi_row = 1 as u8; }
                        break '__c56;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            {
                let __p = unsafe { &mut (*p).n_row };
                let __t = *__p;
                *__p += 1;
                __t
            };
            break '__c55;
        }
        if !(unsafe { sqlite3_step(unsafe { (*p).p_stmt }) } == 100 &&
                        unsafe { (*p).i_err } == 0) {
            break '__b55;
        }
    }
    if unsafe { (*p).i_err } != 0 { qrf_col_data_free(&mut data); return; }
    if unsafe { (*p).spec.b_titles } as i32 == 1 {
        qrf_load_alignment(&data, unsafe { &*p });
    } else {
        let mut e: u8 = 0 as u8;
        if unsafe { (*p).spec.e_title_align } as i32 == 0 {
            e = 2 as u8;
        } else { e = unsafe { (*p).spec.e_title_align }; }
        {
            i = 0 as Sqlite3Int64;
            '__b57: loop {
                if !(i < n_column as i64) { break '__b57; }
                '__c57: loop {
                    unsafe { (*data.a.offset(i as isize)).e = e };
                    break '__c57;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    {
        i = 0 as Sqlite3Int64;
        '__b58: loop {
            if !(i < n_column as i64) { break '__b58; }
            '__c58: loop {
                let mut w: i32 = 0;
                if i < unsafe { (*p).spec.n_width } as i64 {
                    w =
                        unsafe { *unsafe { (*p).spec.a_width.offset(i as isize) } }
                            as i32;
                    if w == -32768 {
                        w = 0;
                        if unsafe { (*p).spec.n_align } as Sqlite3Int64 > i &&
                                unsafe { *unsafe { (*p).spec.a_align.offset(i as isize) } }
                                            as i32 & 3 == 0 {
                            unsafe { (*data.a.offset(i as isize)).e |= 3 as u8 };
                        }
                    } else if w < 0 {
                        w = -w;
                        if unsafe { (*p).spec.n_align } as Sqlite3Int64 > i &&
                                unsafe { *unsafe { (*p).spec.a_align.offset(i as isize) } }
                                            as i32 & 3 == 0 {
                            unsafe { (*data.a.offset(i as isize)).e |= 3 as u8 };
                        }
                    }
                    if w != 0 {
                        unsafe { (*data.a.offset(i as isize)).fx = 1 as u8 };
                    }
                }
                if w == 0 {
                    w = unsafe { (*data.a.offset(i as isize)).mx_w };
                    if unsafe { (*p).spec.n_wrap } as i32 > 0 &&
                            w > unsafe { (*p).spec.n_wrap } as i32 {
                        w = unsafe { (*p).spec.n_wrap } as i32;
                        data.b_multi_row = 1 as u8;
                    }
                } else if (data.b_multi_row as i32 == 0 || w == 1) &&
                        unsafe { (*data.a.offset(i as isize)).mx_w } > w {
                    data.b_multi_row = 1 as u8;
                    if w == 1 {

                        /// If aiWth[j] is 2 or more, then there might be a double-wide
                        ///* character somewhere.  So make the column width at least 2.
                        (w = 2);
                    }
                }
                unsafe { (*data.a.offset(i as isize)).w = w };
                break '__c58;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_column == 1 && data.n > 1 as i64 &&
                        unsafe { (*p).spec.b_split_column } as i32 == 2 &&
                    unsafe { (*p).spec.e_style } as i32 == 2 &&
                unsafe { (*p).spec.b_titles } as i32 == 1 &&
            unsafe { (*p).spec.n_screen_width } as i32 >
                unsafe { (*data.a.offset(0 as isize)).w } + 3 {

        /// Attempt to convert single-column tables into multi-column by
        ///* verticle wrapping, if the screen is wide enough and if the
        ///* bSplitColumn flag is set.
        qrf_split_column(&mut data, p);
        n_column = data.n_col;
    } else {

        /// Adjust the column widths due to screen width restrictions
        qrf_restrict_screen_width(&mut data, unsafe { &*p });
    }
    '__s59:
        {
        match unsafe { (*p).spec.e_style } {
            1 => {
                if data.n_margin != 0 {
                    row_start = c"\u{2502} ".as_ptr() as *mut i8 as *const i8;
                    col_sep = c" \u{2502} ".as_ptr() as *mut i8 as *const i8;
                    row_sep = c" \u{2502}\n".as_ptr() as *mut i8 as *const i8;
                } else {
                    row_start = c"\u{2502}".as_ptr() as *mut i8 as *const i8;
                    col_sep = c"\u{2502}".as_ptr() as *mut i8 as *const i8;
                    row_sep = c"\u{2502}\n".as_ptr() as *mut i8 as *const i8;
                }
                if unsafe { (*p).spec.b_border } as i32 == 1 {
                    {
                        let __n = 3;
                        let __p = &mut row_start;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    row_sep = c"\n".as_ptr() as *mut i8 as *const i8;
                } else {
                    qrf_box_separator(unsafe { (*p).p_out }, &data,
                        c"\u{256d}".as_ptr() as *mut i8 as *const i8,
                        c"\u{252c}".as_ptr() as *mut i8 as *const i8,
                        c"\u{256e}".as_ptr() as *mut i8 as *const i8, 0);
                }
            }
            19 => {
                if data.n_margin != 0 {
                    row_start = c"| ".as_ptr() as *mut i8 as *const i8;
                    col_sep = c" | ".as_ptr() as *mut i8 as *const i8;
                    row_sep = c" |\n".as_ptr() as *mut i8 as *const i8;
                } else {
                    row_start = c"|".as_ptr() as *mut i8 as *const i8;
                    col_sep = c"|".as_ptr() as *mut i8 as *const i8;
                    row_sep = c"|\n".as_ptr() as *mut i8 as *const i8;
                }
                if unsafe { (*p).spec.b_border } as i32 == 1 {
                    {
                        let __n = 1;
                        let __p = &mut row_start;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    row_sep = c"\n".as_ptr() as *mut i8 as *const i8;
                } else {
                    qrf_row_separator(unsafe { (*p).p_out }, &data,
                        '+' as i32 as i8);
                }
            }
            2 => {
                {
                    row_start = c"".as_ptr() as *mut i8 as *const i8;
                    if (data.n_margin as i32) < 2 {
                        col_sep = c" ".as_ptr() as *mut i8 as *const i8;
                    } else if data.n_margin as i32 <= 5 {
                        col_sep = &z_space[(5 - data.n_margin as i32) as usize];
                    } else {
                        col_sep = &raw const z_space[0 as usize] as *const i8;
                    }
                    row_sep = c"\n".as_ptr() as *mut i8 as *const i8;
                    break '__s59;
                }
                if data.n_margin != 0 {
                    row_start = c"| ".as_ptr() as *mut i8 as *const i8;
                    col_sep = c" | ".as_ptr() as *mut i8 as *const i8;
                    row_sep = c" |\n".as_ptr() as *mut i8 as *const i8;
                } else {
                    row_start = c"|".as_ptr() as *mut i8 as *const i8;
                    col_sep = c"|".as_ptr() as *mut i8 as *const i8;
                    row_sep = c"|\n".as_ptr() as *mut i8 as *const i8;
                }
            }
            _ => {
                if data.n_margin != 0 {
                    row_start = c"| ".as_ptr() as *mut i8 as *const i8;
                    col_sep = c" | ".as_ptr() as *mut i8 as *const i8;
                    row_sep = c" |\n".as_ptr() as *mut i8 as *const i8;
                } else {
                    row_start = c"|".as_ptr() as *mut i8 as *const i8;
                    col_sep = c"|".as_ptr() as *mut i8 as *const i8;
                    row_sep = c"|\n".as_ptr() as *mut i8 as *const i8;
                }
            }
        }
    }
    sz_row_start = unsafe { strlen(row_start) } as i32;
    sz_row_sep = unsafe { strlen(row_sep) } as i32;
    sz_col_sep = unsafe { strlen(col_sep) } as i32;
    b_ww =
        (unsafe { (*p).spec.b_word_wrap } as i32 == 2 &&
                data.b_multi_row != 0) as i32;
    if unsafe { (*p).spec.e_style } as i32 == 2 ||
            unsafe { (*p).spec.b_border } as i32 == 1 &&
                (unsafe { (*p).spec.e_style } as i32 == 1 ||
                    unsafe { (*p).spec.e_style } as i32 == 19) {
        b_r_trim = 1;
    } else { b_r_trim = 0; }
    {
        i = 0 as Sqlite3Int64;
        '__b60: loop {
            if !(i < data.n &&
                            unsafe { sqlite3_str_errcode(unsafe { (*p).p_out }) } == 0)
                {
                break '__b60;
            }
            '__c60: loop {
                let mut b_more: i32 = 0;
                let mut n_row: i32 = 0;
                {
                    j = 0 as Sqlite3Int64;
                    '__b61: loop {
                        if !(j < n_column as i64) { break '__b61; }
                        '__c61: loop {
                            unsafe {
                                (*data.a.offset(j as isize)).z =
                                    unsafe { *data.az.offset((i + j) as isize) }
                            };
                            if unsafe { (*data.a.offset(j as isize)).z } ==
                                    core::ptr::null_mut() {
                                unsafe {
                                    (*data.a.offset(j as isize)).z = c"".as_ptr() as *mut i8
                                };
                            }
                            unsafe {
                                (*data.a.offset(j as isize)).b_num =
                                    unsafe { *data.ab_num.offset((i + j) as isize) }
                            };
                            break '__c61;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                '__b62: loop {
                    '__c62: loop {
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out }, row_start,
                                sz_row_start)
                        };
                        b_more = 0;
                        {
                            j = 0 as Sqlite3Int64;
                            '__b63: loop {
                                if !(j < n_column as i64) { break '__b63; }
                                '__c63: loop {
                                    let mut n_this: i32 = 0;
                                    let mut n_wide: i32 = 0;
                                    let mut i_next: i32 = 0;
                                    let mut n_ws: i32 = 0;
                                    qrf_wrap_line(unsafe { (*data.a.offset(j as isize)).z } as
                                            *const i8, unsafe { (*data.a.offset(j as isize)).w }, b_ww,
                                        &mut n_this, &mut n_wide, &mut i_next);
                                    n_ws = unsafe { (*data.a.offset(j as isize)).w } - n_wide;
                                    qrf_print_aligned(unsafe { (*p).p_out },
                                        unsafe { &*data.a.offset(j as isize) }, n_this, n_ws);
                                    {
                                        let __n = i_next;
                                        let __p = unsafe { &mut (*data.a.offset(j as isize)).z };
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    if unsafe {
                                                    *unsafe {
                                                            (*data.a.offset(j as isize)).z.offset(0 as isize)
                                                        }
                                                } as i32 != 0 {
                                        b_more = 1;
                                    }
                                    if j < (n_column - 1) as i64 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out }, col_sep,
                                                sz_col_sep)
                                        };
                                    } else {
                                        if b_r_trim != 0 { qrf_r_trim(unsafe { (*p).p_out }); }
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out }, row_sep,
                                                sz_row_sep)
                                        };
                                    }
                                    break '__c63;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        break '__c62;
                    }
                    if !(b_more != 0 &&
                                    { let __p = &mut n_row; *__p += 1; *__p } <
                                        unsafe { (*p).mx_height }) {
                        break '__b62;
                    }
                }
                if b_more != 0 {

                    /// This row was terminated by nLineLimit.  Show ellipsis.
                    unsafe {
                        sqlite3_str_append(unsafe { (*p).p_out }, row_start,
                            sz_row_start)
                    };
                    {
                        j = 0 as Sqlite3Int64;
                        '__b64: loop {
                            if !(j < n_column as i64) { break '__b64; }
                            '__c64: loop {
                                if unsafe {
                                                *unsafe {
                                                        (*data.a.offset(j as isize)).z.offset(0 as isize)
                                                    }
                                            } as i32 == 0 {
                                    unsafe {
                                        sqlite3_str_appendchar(unsafe { (*p).p_out },
                                            unsafe { (*data.a.offset(j as isize)).w }, ' ' as i32 as i8)
                                    };
                                } else {
                                    let mut n_e: i32 = 3;
                                    if n_e > unsafe { (*data.a.offset(j as isize)).w } {
                                        n_e = unsafe { (*data.a.offset(j as isize)).w };
                                    }
                                    unsafe {
                                        (*data.a.offset(j as isize)).z = c"...".as_ptr() as *mut i8
                                    };
                                    qrf_print_aligned(unsafe { (*p).p_out },
                                        unsafe { &*data.a.offset(j as isize) }, n_e,
                                        unsafe { (*data.a.offset(j as isize)).w } - n_e);
                                }
                                if j < (n_column - 1) as i64 {
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out }, col_sep,
                                            sz_col_sep)
                                    };
                                } else {
                                    if b_r_trim != 0 { qrf_r_trim(unsafe { (*p).p_out }); }
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out }, row_sep,
                                            sz_row_sep)
                                    };
                                }
                                break '__c64;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                if (i == 0 as i64 || data.b_multi_row != 0) &&
                        (i + n_column as Sqlite3Int64) < data.n {
                    let is_title_data_separator: i32 =
                        (i == 0 as i64 && unsafe { (*p).spec.b_titles } as i32 == 2)
                            as i32;
                    if is_title_data_separator != 0 {
                        qrf_load_alignment(&data, unsafe { &*p });
                    }
                    '__s65:
                        {
                        match unsafe { (*p).spec.e_style } {
                            19 => {
                                {
                                    if is_title_data_separator != 0 || data.b_multi_row != 0 {
                                        qrf_row_separator(unsafe { (*p).p_out }, &data,
                                            '+' as i32 as i8);
                                    }
                                    break '__s65;
                                }
                                {
                                    if is_title_data_separator != 0 {
                                        qrf_box_separator(unsafe { (*p).p_out }, &data,
                                            c"\u{255e}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{256a}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{2561}".as_ptr() as *mut i8 as *const i8, 1);
                                    } else if data.b_multi_row != 0 {
                                        qrf_box_separator(unsafe { (*p).p_out }, &data,
                                            c"\u{251c}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{253c}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{2524}".as_ptr() as *mut i8 as *const i8, 0);
                                    }
                                    break '__s65;
                                }
                                {
                                    if is_title_data_separator != 0 {
                                        qrf_row_separator(unsafe { (*p).p_out }, &data,
                                            '|' as i32 as i8);
                                    }
                                    break '__s65;
                                }
                                {
                                    if is_title_data_separator != 0 {
                                        {
                                            j = 0 as Sqlite3Int64;
                                            '__b66: loop {
                                                if !(j < n_column as i64) { break '__b66; }
                                                '__c66: loop {
                                                    unsafe {
                                                        sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                            unsafe { (*data.a.offset(j as isize)).w }, '-' as i32 as i8)
                                                    };
                                                    if j < (n_column - 1) as i64 {
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, col_sep,
                                                                sz_col_sep)
                                                        };
                                                    } else {
                                                        qrf_r_trim(unsafe { (*p).p_out });
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, row_sep,
                                                                sz_row_sep)
                                                        };
                                                    }
                                                    break '__c66;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                    } else if data.b_multi_row != 0 {
                                        qrf_r_trim(unsafe { (*p).p_out });
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    break '__s65;
                                }
                            }
                            1 => {
                                {
                                    if is_title_data_separator != 0 {
                                        qrf_box_separator(unsafe { (*p).p_out }, &data,
                                            c"\u{255e}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{256a}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{2561}".as_ptr() as *mut i8 as *const i8, 1);
                                    } else if data.b_multi_row != 0 {
                                        qrf_box_separator(unsafe { (*p).p_out }, &data,
                                            c"\u{251c}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{253c}".as_ptr() as *mut i8 as *const i8,
                                            c"\u{2524}".as_ptr() as *mut i8 as *const i8, 0);
                                    }
                                    break '__s65;
                                }
                                {
                                    if is_title_data_separator != 0 {
                                        qrf_row_separator(unsafe { (*p).p_out }, &data,
                                            '|' as i32 as i8);
                                    }
                                    break '__s65;
                                }
                                {
                                    if is_title_data_separator != 0 {
                                        {
                                            j = 0 as Sqlite3Int64;
                                            '__b66: loop {
                                                if !(j < n_column as i64) { break '__b66; }
                                                '__c66: loop {
                                                    unsafe {
                                                        sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                            unsafe { (*data.a.offset(j as isize)).w }, '-' as i32 as i8)
                                                    };
                                                    if j < (n_column - 1) as i64 {
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, col_sep,
                                                                sz_col_sep)
                                                        };
                                                    } else {
                                                        qrf_r_trim(unsafe { (*p).p_out });
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, row_sep,
                                                                sz_row_sep)
                                                        };
                                                    }
                                                    break '__c66;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                    } else if data.b_multi_row != 0 {
                                        qrf_r_trim(unsafe { (*p).p_out });
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    break '__s65;
                                }
                            }
                            13 => {
                                {
                                    if is_title_data_separator != 0 {
                                        qrf_row_separator(unsafe { (*p).p_out }, &data,
                                            '|' as i32 as i8);
                                    }
                                    break '__s65;
                                }
                                {
                                    if is_title_data_separator != 0 {
                                        {
                                            j = 0 as Sqlite3Int64;
                                            '__b66: loop {
                                                if !(j < n_column as i64) { break '__b66; }
                                                '__c66: loop {
                                                    unsafe {
                                                        sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                            unsafe { (*data.a.offset(j as isize)).w }, '-' as i32 as i8)
                                                    };
                                                    if j < (n_column - 1) as i64 {
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, col_sep,
                                                                sz_col_sep)
                                                        };
                                                    } else {
                                                        qrf_r_trim(unsafe { (*p).p_out });
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, row_sep,
                                                                sz_row_sep)
                                                        };
                                                    }
                                                    break '__c66;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                    } else if data.b_multi_row != 0 {
                                        qrf_r_trim(unsafe { (*p).p_out });
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    break '__s65;
                                }
                            }
                            2 => {
                                {
                                    if is_title_data_separator != 0 {
                                        {
                                            j = 0 as Sqlite3Int64;
                                            '__b66: loop {
                                                if !(j < n_column as i64) { break '__b66; }
                                                '__c66: loop {
                                                    unsafe {
                                                        sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                            unsafe { (*data.a.offset(j as isize)).w }, '-' as i32 as i8)
                                                    };
                                                    if j < (n_column - 1) as i64 {
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, col_sep,
                                                                sz_col_sep)
                                                        };
                                                    } else {
                                                        qrf_r_trim(unsafe { (*p).p_out });
                                                        unsafe {
                                                            sqlite3_str_append(unsafe { (*p).p_out }, row_sep,
                                                                sz_row_sep)
                                                        };
                                                    }
                                                    break '__c66;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                    } else if data.b_multi_row != 0 {
                                        qrf_r_trim(unsafe { (*p).p_out });
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    break '__s65;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                break '__c60;
            }
            i += n_column as Sqlite3Int64;
        }
    }
    if unsafe { (*p).spec.b_border } as i32 != 1 {
        '__s67:
            {
            match unsafe { (*p).spec.e_style } {
                1 => {
                    qrf_box_separator(unsafe { (*p).p_out }, &data,
                        c"\u{2570}".as_ptr() as *mut i8 as *const i8,
                        c"\u{2534}".as_ptr() as *mut i8 as *const i8,
                        c"\u{256f}".as_ptr() as *mut i8 as *const i8, 0);
                }
                19 => {
                    qrf_row_separator(unsafe { (*p).p_out }, &data,
                        '+' as i32 as i8);
                }
                _ => {}
            }
        }
    }
    qrf_write(p);
    qrf_col_data_free(&mut data);
    return;
}

///* Parameter azArray points to a zero-terminated array of strings. zStr
///* points to a single nul-terminated string. Return non-zero if zStr
///* is equal, according to strcmp(), to any of the strings in the array.
///* Otherwise, return zero.
extern "C" fn qrf_string_in_array(z_str_1: *const i8,
    az_array_1: *const *const i8) -> i32 {
    let mut i: i32 = 0;
    if z_str_1 == core::ptr::null() { return 0; }
    {
        i = 0;
        '__b68: loop {
            if !(!(unsafe { *az_array_1.offset(i as isize) }).is_null()) {
                break '__b68;
            }
            '__c68: loop {
                if 0 ==
                        unsafe {
                            strcmp(z_str_1, unsafe { *az_array_1.offset(i as isize) })
                        } {
                    return 1;
                }
                break '__c68;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Store string zUtf to pOut as w characters.  If w is negative,
///* then right-justify the text.  W is the width in display characters, not
///* in bytes.  Double-width unicode characters count as two characters.
///* VT100 escape sequences count as zero.  And so forth.
extern "C" fn qrf_width_print(p: *const Qrf, p_out_1: *mut Sqlite3Str,
    mut w: i32, z_utf_1: *const i8) -> () {
    let mut a: *const u8 = z_utf_1 as *const u8;
    let mut c: u8 = 0 as u8;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut aw: i32 = 0;
    { let _ = p; };
    if w < -mx_w_1 { w = -mx_w_1; } else if w > mx_w_1 { w = mx_w_1 as i32; }
    aw = if w < 0 { -w } else { w };
    if a == core::ptr::null() { a = c"".as_ptr() as *const u8; }
    while { c = unsafe { *a.offset(i as isize) } as u8; c } as i32 != 0 {
        if c as i32 & 192 == 192 {
            let mut u: i32 = 0;
            let len: i32 =
                sqlite3_qrf_decode_utf8(unsafe { a.offset(i as isize) },
                    &mut u);
            let x: i32 = sqlite3_qrf_wcwidth(u);
            if x + n > aw { break; }
            i += len;
            n += x;
        } else if c as i32 == 27 &&
                { k = qrf_is_vt100(unsafe { &*a.offset(i as isize) }); k } > 0
            {
            i += k;
        } else if n >= aw {
            break;
        } else {
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n >= aw {
        unsafe { sqlite3_str_append(p_out_1, z_utf_1, i) };
    } else if w < 0 {
        if aw > n {
            unsafe {
                sqlite3_str_appendchar(p_out_1, aw - n, ' ' as i32 as i8)
            };
        }
        unsafe { sqlite3_str_append(p_out_1, z_utf_1, i) };
    } else {
        unsafe { sqlite3_str_append(p_out_1, z_utf_1, i) };
        if aw > n {
            unsafe {
                sqlite3_str_appendchar(p_out_1, aw - n, ' ' as i32 as i8)
            };
        }
    }
}

///* Return an estimate of the number of display columns used by the
///* string in the argument.  The width of individual characters is
///* determined as for sqlite3_qrf_wcwidth().  VT100 escape code sequences
///* are assigned a width of zero.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_qrf_wcswidth(z_in: *const i8) -> u64 {
    let mut z: *const u8 = z_in as *const u8;
    let mut n: u64 = 0 as u64;
    while unsafe { *z } != 0 {
        if (unsafe { *z.offset(0 as isize) } as i32) < ' ' as i32 {
            let mut k: i32 = 0;
            if unsafe { *z.offset(0 as isize) } as i32 == '\u{1b}' as i32 &&
                    { k = qrf_is_vt100(z); k } > 0 {
                {
                    let __n = k;
                    let __p = &mut z;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            } else {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        } else if 128 & unsafe { *z.offset(0 as isize) } as i32 == 0 {
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else {
            let mut u: i32 = 0;
            let len: i32 = sqlite3_qrf_decode_utf8(z, &mut u);
            {
                let __n = len;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += sqlite3_qrf_wcwidth(u) as u64;
        }
    }
    return n;
}

///* Print out an EXPLAIN with indentation.  This is a two-pass algorithm.
///*
///* On the first pass, we compute aiIndent[iOp] which is the amount of
///* indentation to apply to the iOp-th opcode.  The output actually occurs
///* on the second pass.
///*
///* The indenting rules are:
///*
///*     * For each "Next", "Prev", "VNext" or "VPrev" instruction, indent
///*       all opcodes that occur between the p2 jump destination and the opcode
///*       itself by 2 spaces.
///*
///*     * Do the previous for "Return" instructions for when P2 is positive.
///*       See tag-20220407a in wherecode.c and vdbe.c.
///*
///*     * For each "Goto", if the jump destination is earlier in the program
///*       and ends on one of:
///*          Yield  SeekGt  SeekLt  RowSetRead  Rewind
///*       or if the P1 parameter is one instead of zero,
///*       then indent all opcodes between the earlier instruction
///*       and "Goto" by 2 spaces.
#[allow(unused_doc_comments)]
extern "C" fn qrf_explain(p: *mut Qrf) -> () {
    let mut ab_yield: *mut i32 = core::ptr::null_mut();
    /// abYield[iOp] is rue if opcode iOp is an OP_Yield
    let mut ai_indent: *mut i32 = core::ptr::null_mut();
    /// Indent the iOp-th opcode by aiIndent[iOp]
    let mut n_alloc: i64 = 0 as i64;
    /// Allocated size of aiIndent[], abYield
    let mut n_indent: i32 = 0;
    /// Number of entries in aiIndent[]
    let mut i_op: i32 = 0;
    /// Opcode number
    let mut i: i32 = 0;
    /// Column loop counter
    let mut az_next: [*const i8; 7] =
        [c"Next".as_ptr() as *const i8, c"Prev".as_ptr() as *const i8,
                c"VPrev".as_ptr() as *const i8,
                c"VNext".as_ptr() as *const i8,
                c"SorterNext".as_ptr() as *const i8,
                c"Return".as_ptr() as *const i8, core::ptr::null()];
    let mut az_yield: [*const i8; 6] =
        [c"Yield".as_ptr() as *const i8, c"SeekLT".as_ptr() as *const i8,
                c"SeekGT".as_ptr() as *const i8,
                c"RowSetRead".as_ptr() as *const i8,
                c"Rewind".as_ptr() as *const i8, core::ptr::null()];
    let mut az_goto: [*const i8; 2] =
        [c"Goto".as_ptr() as *const i8, core::ptr::null()];

    /// The caller guarantees that the leftmost 4 columns of the statement
    ///* passed to this function are equivalent to the leftmost 4 columns
    ///* of EXPLAIN statement output. In practice the statement may be
    ///* an EXPLAIN, or it may be a query on the bytecode() virtual table.
    if !(unsafe { sqlite3_column_count(unsafe { (*p).p_stmt }) } >= 4) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"qrfExplain".as_ptr() as *const i8,
                c"qrf.c".as_ptr() as *mut i8 as *const i8, 2348,
                c"sqlite3_column_count(p->pStmt)>=4".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(0 ==
                            unsafe {
                                sqlite3_stricmp(unsafe {
                                        sqlite3_column_name(unsafe { (*p).p_stmt }, 0)
                                    }, c"addr".as_ptr() as *mut i8 as *const i8)
                            }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"qrfExplain".as_ptr() as *const i8,
                c"qrf.c".as_ptr() as *mut i8 as *const i8, 2349,
                c"0==sqlite3_stricmp( sqlite3_column_name(p->pStmt, 0), \"addr\" )".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(0 ==
                            unsafe {
                                sqlite3_stricmp(unsafe {
                                        sqlite3_column_name(unsafe { (*p).p_stmt }, 1)
                                    }, c"opcode".as_ptr() as *mut i8 as *const i8)
                            }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"qrfExplain".as_ptr() as *const i8,
                c"qrf.c".as_ptr() as *mut i8 as *const i8, 2350,
                c"0==sqlite3_stricmp( sqlite3_column_name(p->pStmt, 1), \"opcode\" )".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(0 ==
                            unsafe {
                                sqlite3_stricmp(unsafe {
                                        sqlite3_column_name(unsafe { (*p).p_stmt }, 2)
                                    }, c"p1".as_ptr() as *mut i8 as *const i8)
                            }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"qrfExplain".as_ptr() as *const i8,
                c"qrf.c".as_ptr() as *mut i8 as *const i8, 2351,
                c"0==sqlite3_stricmp( sqlite3_column_name(p->pStmt, 2), \"p1\" )".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(0 ==
                            unsafe {
                                sqlite3_stricmp(unsafe {
                                        sqlite3_column_name(unsafe { (*p).p_stmt }, 3)
                                    }, c"p2".as_ptr() as *mut i8 as *const i8)
                            }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"qrfExplain".as_ptr() as *const i8,
                c"qrf.c".as_ptr() as *mut i8 as *const i8, 2352,
                c"0==sqlite3_stricmp( sqlite3_column_name(p->pStmt, 3), \"p2\" )".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        i_op = 0;
        '__b71: loop {
            if !(100 == unsafe { sqlite3_step(unsafe { (*p).p_stmt }) } &&
                            (unsafe { (*p).i_err } == 0) as i32 != 0) {
                break '__b71;
            }
            '__c71: loop {
                let i_addr: i32 =
                    unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                let z_op: *const i8 =
                    unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 1) } as
                        *const i8;
                let p1: i32 =
                    unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 2) };
                let p2: i32 =
                    unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 3) };
                /// Assuming that p2 is an instruction address, set variable p2op to the
                ///* index of that instruction in the aiIndent[] array. p2 and p2op may be
                ///* different if the current instruction is part of a sub-program generated
                ///* by an SQL trigger or foreign key.
                let p2op: i32 = p2 + (i_op - i_addr);
                if i_op as i64 >= n_alloc {
                    n_alloc += 100 as i64;
                    ai_indent =
                        unsafe {
                                sqlite3_realloc64(ai_indent as *mut (),
                                    n_alloc as u64 * core::mem::size_of::<i32>() as u64)
                            } as *mut i32;
                    ab_yield =
                        unsafe {
                                sqlite3_realloc64(ab_yield as *mut (),
                                    n_alloc as u64 * core::mem::size_of::<i32>() as u64)
                            } as *mut i32;
                    if ai_indent == core::ptr::null_mut() ||
                            ab_yield == core::ptr::null_mut() {
                        qrf_oom(p);
                        unsafe { sqlite3_free(ai_indent as *mut ()) };
                        unsafe { sqlite3_free(ab_yield as *mut ()) };
                        return;
                    }
                }
                unsafe {
                    *ab_yield.offset(i_op as isize) =
                        qrf_string_in_array(z_op,
                            &raw mut az_yield[0 as usize] as *mut *const i8 as
                                *const *const i8)
                };
                unsafe { *ai_indent.offset(i_op as isize) = 0 };
                n_indent = i_op + 1;
                if qrf_string_in_array(z_op,
                                &raw mut az_next[0 as usize] as *mut *const i8 as
                                    *const *const i8) != 0 && p2op > 0 {
                    {
                        i = p2op;
                        '__b72: loop {
                            if !(i < i_op) { break '__b72; }
                            '__c72: loop {
                                unsafe { *ai_indent.offset(i as isize) += 2 };
                                break '__c72;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                if qrf_string_in_array(z_op,
                                    &raw mut az_goto[0 as usize] as *mut *const i8 as
                                        *const *const i8) != 0 && p2op < i_op &&
                        (unsafe { *ab_yield.offset(p2op as isize) } != 0 || p1 != 0)
                    {
                    {
                        i = p2op;
                        '__b73: loop {
                            if !(i < i_op) { break '__b73; }
                            '__c73: loop {
                                unsafe { *ai_indent.offset(i as isize) += 2 };
                                break '__c73;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                break '__c71;
            }
            { let __p = &mut i_op; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(ab_yield as *mut ()) };

    /// Second pass.  Actually generate output
    unsafe { sqlite3_reset(unsafe { (*p).p_stmt }) };
    if unsafe { (*p).i_err } == 0 {
        let mut a_width: *const i32 =
            &raw const a_explain_width[0 as usize] as *const i32;
        let mut a_map: *const i32 =
            &raw const a_explain_map[0 as usize] as *const i32;
        let mut n_width: i32 =
            (core::mem::size_of::<[i32; 8]>() as u64 /
                    core::mem::size_of::<i32>() as u64) as i32;
        let mut i_indent: i32 = 1;
        let mut n_arg: i32 = unsafe { (*p).n_col };
        if unsafe { (*p).spec.e_style } as i32 == 18 {
            a_width = &raw const a_scan_exp_width[0 as usize] as *const i32;
            a_map = &raw const a_scan_exp_map[0 as usize] as *const i32;
            n_width =
                (core::mem::size_of::<[i32; 10]>() as u64 /
                        core::mem::size_of::<i32>() as u64) as i32;
            i_indent = 3;
        }
        if n_arg > n_width { n_arg = n_width; }
        {
            i_op = 0;
            '__b74: loop {
                if !(unsafe { sqlite3_step(unsafe { (*p).p_stmt }) } == 100 &&
                                (unsafe { (*p).i_err } == 0) as i32 != 0) {
                    break '__b74;
                }
                '__c74: loop {
                    if i_op == 0 {
                        {
                            i = 0;
                            '__b75: loop {
                                if !(i < n_arg) { break '__b75; }
                                '__c75: loop {
                                    let z_col: *const i8 =
                                        unsafe {
                                            sqlite3_column_name(unsafe { (*p).p_stmt },
                                                unsafe { *a_map.offset(i as isize) })
                                        };
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        unsafe { *a_width.offset(i as isize) }, z_col);
                                    if i == n_arg - 1 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"  ".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    break '__c75;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        {
                            i = 0;
                            '__b76: loop {
                                if !(i < n_arg) { break '__b76; }
                                '__c76: loop {
                                    unsafe {
                                        sqlite3_str_appendf(unsafe { (*p).p_out },
                                            c"%.*c".as_ptr() as *mut i8 as *const i8,
                                            unsafe { *a_width.offset(i as isize) }, '-' as i32)
                                    };
                                    if i == n_arg - 1 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"  ".as_ptr() as *mut i8 as *const i8, 2)
                                        };
                                    }
                                    break '__c76;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                    {
                        i = 0;
                        '__b77: loop {
                            if !(i < n_arg) { break '__b77; }
                            '__c77: loop {
                                let mut z_sep: *const i8 =
                                    c"  ".as_ptr() as *mut i8 as *const i8;
                                let mut w: i32 =
                                    unsafe { *a_width.offset(i as isize) } as i32;
                                let mut z_val: *const i8 =
                                    unsafe {
                                            sqlite3_column_text(unsafe { (*p).p_stmt },
                                                unsafe { *a_map.offset(i as isize) })
                                        } as *const i8;
                                let mut len: i32 = 0;
                                if i == n_arg - 1 { w = 0; }
                                if z_val == core::ptr::null() {
                                    z_val = c"".as_ptr() as *mut i8 as *const i8;
                                }
                                len = sqlite3_qrf_wcswidth(z_val) as i32;
                                if len > w {
                                    w = len;
                                    z_sep = c" ".as_ptr() as *mut i8 as *const i8;
                                }
                                if i == i_indent && !(ai_indent).is_null() &&
                                        i_op < n_indent {
                                    unsafe {
                                        sqlite3_str_appendchar(unsafe { (*p).p_out },
                                            unsafe { *ai_indent.offset(i_op as isize) },
                                            ' ' as i32 as i8)
                                    };
                                }
                                qrf_width_print(p as *const Qrf, unsafe { (*p).p_out }, w,
                                    z_val);
                                if i == n_arg - 1 {
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_str_appendall(unsafe { (*p).p_out }, z_sep)
                                    };
                                }
                                break '__c77;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    {
                        let __p = unsafe { &mut (*p).n_row };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    break '__c74;
                }
                { let __p = &mut i_op; let __t = *__p; *__p += 1; __t };
            }
        }
        qrf_write(p);
    }
    unsafe { sqlite3_free(ai_indent as *mut ()) };
}

///* Do a "scanstatus vm" style EXPLAIN listing on p->pStmt.
///*
///* p->pStmt is probably not an EXPLAIN query.  Instead, construct a
///* new query that is a bytecode() rendering of p->pStmt with extra
///* columns for the "scanstatus vm" outputs, and run the results of
///* that new query through the normal EXPLAIN formatting.
extern "C" fn qrf_scan_status_vm(p: *mut Qrf) -> () {
    unsafe {
        let p_orig_stmt: *mut Sqlite3Stmt = unsafe { (*p).p_stmt };
        let mut p_explain: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut rc: i32 = 0;
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p).db }, z_sql, -1,
                    &mut p_explain, core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                qrf_error(unsafe { &mut *p }, rc,
                    c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
            };
            unsafe { sqlite3_finalize(p_explain) };
            return;
        }
        unsafe {
            sqlite3_bind_pointer(p_explain, 1, p_orig_stmt as *mut (),
                c"stmt-pointer".as_ptr() as *mut i8 as *const i8, None)
        };
        unsafe { (*p).p_stmt = p_explain };
        unsafe { (*p).n_col = 10 };
        qrf_explain(p);
        unsafe { sqlite3_finalize(p_explain) };
        unsafe { (*p).p_stmt = p_orig_stmt };
    }
}

///* Generate ".scanstatus est" style of EQP output.
extern "C" fn qrf_eqp_stats(p: *mut Qrf) -> () {
    unsafe {
        qrf_error(unsafe { &mut *p }, 1,
            c"not available in this build".as_ptr() as *mut i8 as *const i8)
    };
}

///* Helper function for QRF_STYLE_Json and QRF_STYLE_JObject.
///* The initial "{" for a JSON object that will contain row content
///* has been output.  Now output all the content.
extern "C" fn qrf_one_json_row(p: *mut Qrf) -> () {
    let mut i: i32 = 0;
    let mut n_item: i32 = 0;
    {
        n_item = { i = 0; i };
        '__b78: loop {
            if !(i < unsafe { (*p).n_col }) { break '__b78; }
            '__c78: loop {
                let mut z_c_name: *const i8 = core::ptr::null();
                z_c_name =
                    unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                if n_item > 0 {
                    unsafe {
                        sqlite3_str_append(unsafe { (*p).p_out },
                            c",".as_ptr() as *mut i8 as *const i8, 1)
                    };
                }
                { let __p = &mut n_item; let __t = *__p; *__p += 1; __t };
                qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name);
                unsafe {
                    sqlite3_str_append(unsafe { (*p).p_out },
                        c":".as_ptr() as *mut i8 as *const i8, 1)
                };
                qrf_render_value(p, unsafe { (*p).p_out }, i);
                break '__c78;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    qrf_write(p);
}

///* Attempt to determine if identifier zName needs to be quoted, either
///* because it contains non-alphanumeric characters, or because it is an
///* SQLite keyword.  Be conservative in this estimate:  When in doubt assume
///* that quoting is required.
///*
///* Return 1 if quoting is required.  Return 0 if no quoting is required.
extern "C" fn qrf_need_quote(z_name_1: *const i8) -> i32 {
    let mut i: i32 = 0;
    let z: *const u8 = z_name_1 as *const u8;
    if z == core::ptr::null() { return 1; }
    if !(qrf_c_type[unsafe { *z.offset(0 as isize) } as u8 as usize] as i32 &
                            4 != 0) as i32 != 0 {
        return 1;
    }
    {
        i = 0;
        '__b79: loop {
            if !(unsafe { *z.offset(i as isize) } != 0) { break '__b79; }
            '__c79: loop {
                if !(qrf_c_type[unsafe { *z.offset(i as isize) } as u8 as
                                                    usize] as i32 & 6 != 0) as i32 != 0 {
                    return 1;
                }
                break '__c79;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return (unsafe { sqlite3_keyword_check(z_name_1, i) } != 0) as i32;
}

///* Free and reset the EXPLAIN QUERY PLAN data that has been collected
///* in p->u.pGraph.
extern "C" fn qrf_eqp_reset(p: &mut Qrf) -> () {
    unsafe {
        let mut p_row: *mut QrfEQPGraphRow = core::ptr::null_mut();
        let mut p_next: *mut QrfEQPGraphRow = core::ptr::null_mut();
        if !((*p).u.p_graph).is_null() {
            {
                p_row = unsafe { (*(*p).u.p_graph).p_row };
                '__b80: loop {
                    if !(!(p_row).is_null()) { break '__b80; }
                    '__c80: loop {
                        p_next = unsafe { (*p_row).p_next };
                        unsafe { sqlite3_free(p_row as *mut ()) };
                        break '__c80;
                    }
                    p_row = p_next;
                }
            }
            unsafe { sqlite3_free((*p).u.p_graph as *mut ()) };
            (*p).u.p_graph = core::ptr::null_mut();
        }
    }
}

///* Render the 64-bit value N in a more human-readable format into
///* pOut.
///*
///*   +  Only show the first three significant digits.
///*   +  Append suffixes K, M, G, T, P, and E for 1e3, 1e6, ... 1e18
extern "C" fn qrf_approx_int64(p_out_1: *mut Sqlite3Str, mut n_1: i64) -> () {
    let mut i: i32 = 0;
    if n_1 < 0 as i64 {
        n_1 =
            if n_1 == -9223372036854775807i64 - 1 as i64 {
                9223372036854775807i64
            } else { -n_1 };
        unsafe {
            sqlite3_str_append(p_out_1, c"-".as_ptr() as *mut i8 as *const i8,
                1)
        };
    }
    if n_1 < 10000 as i64 {
        unsafe {
            sqlite3_str_appendf(p_out_1,
                c"%4lld ".as_ptr() as *mut i8 as *const i8, n_1)
        };
        return;
    }
    if n_1 >= 9223372036854775800i64 {
        unsafe {
            sqlite3_str_appendf(p_out_1,
                c"%.2fE".as_ptr() as *mut i8 as *const i8, 1e-18 * n_1 as f64)
        };
        return;
    }
    {
        i = 1;
        '__b81: loop {
            if !(i <= 18) { break '__b81; }
            '__c81: loop {
                n_1 = (n_1 + 5 as i64) / 10 as i64;
                if n_1 < 10000 as i64 {
                    let n: i32 = n_1 as i32;
                    '__s82:
                        {
                        match i % 3 {
                            0 => {
                                unsafe {
                                    sqlite3_str_appendf(p_out_1,
                                        c"%d.%02d".as_ptr() as *mut i8 as *const i8, n / 1000,
                                        n % 1000 / 10)
                                };
                            }
                            1 => {
                                unsafe {
                                    sqlite3_str_appendf(p_out_1,
                                        c"%2d.%d".as_ptr() as *mut i8 as *const i8, n / 100,
                                        n % 100 / 10)
                                };
                            }
                            2 => {
                                unsafe {
                                    sqlite3_str_appendf(p_out_1,
                                        c"%4d".as_ptr() as *mut i8 as *const i8, n / 10)
                                };
                            }
                            _ => {}
                        }
                    }
                    unsafe {
                        sqlite3_str_append(p_out_1, &a_suffix[(i / 3) as usize], 1)
                    };
                    break '__b81;
                }
                break '__c81;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

/// Return the next EXPLAIN QUERY PLAN line with iEqpId that occurs after
///* pOld, or return the first such line if pOld is NULL
extern "C" fn qrf_eqp_next_row(p: &Qrf, i_eqp_id_1: i32,
    p_old_1: *const QrfEQPGraphRow) -> *mut QrfEQPGraphRow {
    unsafe {
        let mut p_row: *mut QrfEQPGraphRow =
            if !(p_old_1).is_null() {
                unsafe { (*p_old_1).p_next }
            } else { unsafe { (*(*p).u.p_graph).p_row } };
        while !(p_row).is_null() &&
                unsafe { (*p_row).i_parent_id } != i_eqp_id_1 {
            p_row = unsafe { (*p_row).p_next };
        }
        return p_row;
    }
}

/// Render a single level of the graph that has iEqpId as its parent.  Called
///* recursively to render sublevels.
extern "C" fn qrf_eqp_render_level(p: *mut Qrf, i_eqp_id_1: i32) -> () {
    unsafe {
        let mut p_row: *mut QrfEQPGraphRow = core::ptr::null_mut();
        let mut p_next: *mut QrfEQPGraphRow = core::ptr::null_mut();
        let n: i64 =
            unsafe {
                    strlen(unsafe {
                                    &raw mut (*unsafe { (*p).u.p_graph }).z_prefix[0 as usize]
                                } as *mut i8 as *const i8)
                } as i64;
        let mut z: *mut i8 = core::ptr::null_mut();
        {
            p_row =
                qrf_eqp_next_row(unsafe { &*p }, i_eqp_id_1,
                    core::ptr::null());
            '__b84: loop {
                if !(!(p_row).is_null()) { break '__b84; }
                '__c84: loop {
                    p_next =
                        qrf_eqp_next_row(unsafe { &*p }, i_eqp_id_1,
                            p_row as *const QrfEQPGraphRow);
                    z =
                        unsafe { &raw mut (*p_row).z_text[0 as usize] } as *mut i8;
                    unsafe {
                        sqlite3_str_appendf(unsafe { (*p).p_out },
                            c"%s%s%s\n".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                    &raw mut (*unsafe { (*p).u.p_graph }).z_prefix[0 as usize]
                                } as *mut i8,
                            if !(p_next).is_null() {
                                c"|--".as_ptr() as *mut i8
                            } else { c"`--".as_ptr() as *mut i8 }, z)
                    };
                    if n < core::mem::size_of::<[i8; 400]>() as i64 - 7 as i64 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut (*unsafe { (*p).u.p_graph }).z_prefix[n as usize]
                                    } as *mut (),
                                if !(p_next).is_null() {
                                        c"|  ".as_ptr() as *mut i8
                                    } else { c"   ".as_ptr() as *mut i8 } as *const (),
                                4 as u64)
                        };
                        qrf_eqp_render_level(p, unsafe { (*p_row).i_eqp_id });
                        unsafe {
                            (*unsafe { (*p).u.p_graph }).z_prefix[n as usize] = 0 as i8
                        };
                    }
                    break '__c84;
                }
                p_row = p_next;
            }
        }
    }
}

///* Display and reset the EXPLAIN QUERY PLAN data
extern "C" fn qrf_eqp_render(p: *mut Qrf, n_cycle_1: i64) -> () {
    unsafe {
        let mut p_row: *mut QrfEQPGraphRow = core::ptr::null_mut();
        if unsafe { (*p).u.p_graph } != core::ptr::null_mut() &&
                {
                        p_row = unsafe { (*unsafe { (*p).u.p_graph }).p_row };
                        p_row
                    } != core::ptr::null_mut() {
            if unsafe { (*p_row).z_text[0 as usize] } as i32 == '-' as i32 {
                if unsafe { (*p_row).p_next } == core::ptr::null_mut() {
                    qrf_eqp_reset(unsafe { &mut *p });
                    return;
                }
                unsafe {
                    sqlite3_str_appendf(unsafe { (*p).p_out },
                        c"%s\n".as_ptr() as *mut i8 as *const i8,
                        unsafe {
                            (unsafe { &raw mut (*p_row).z_text[0 as usize] } as
                                    *mut i8).offset(3 as isize)
                        })
                };
                unsafe {
                    (*unsafe { (*p).u.p_graph }).p_row =
                        unsafe { (*p_row).p_next }
                };
                unsafe { sqlite3_free(p_row as *mut ()) };
            } else if n_cycle_1 > 0 as i64 {
                let n_sp: i32 =
                    unsafe { (*unsafe { (*p).u.p_graph }).n_width } - 2;
                if unsafe { (*p).spec.e_style } as i32 == 17 {
                    unsafe {
                        sqlite3_str_appendchar(unsafe { (*p).p_out }, n_sp,
                            ' ' as i32 as i8)
                    };
                    unsafe {
                        sqlite3_str_appendall(unsafe { (*p).p_out },
                            c"Cycles      Loops  (est)  Rows   (est)\n".as_ptr() as
                                    *mut i8 as *const i8)
                    };
                    unsafe {
                        sqlite3_str_appendchar(unsafe { (*p).p_out }, n_sp,
                            ' ' as i32 as i8)
                    };
                    unsafe {
                        sqlite3_str_appendall(unsafe { (*p).p_out },
                            c"----------  ------------  ------------\n".as_ptr() as
                                    *mut i8 as *const i8)
                    };
                } else {
                    unsafe {
                        sqlite3_str_appendchar(unsafe { (*p).p_out }, n_sp,
                            ' ' as i32 as i8)
                    };
                    unsafe {
                        sqlite3_str_appendall(unsafe { (*p).p_out },
                            c"Cycles      Loops  Rows \n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    unsafe {
                        sqlite3_str_appendchar(unsafe { (*p).p_out }, n_sp,
                            ' ' as i32 as i8)
                    };
                    unsafe {
                        sqlite3_str_appendall(unsafe { (*p).p_out },
                            c"----------  -----  -----\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                }
                unsafe {
                    sqlite3_str_appendall(unsafe { (*p).p_out },
                        c"QUERY PLAN".as_ptr() as *mut i8 as *const i8)
                };
                unsafe {
                    sqlite3_str_appendchar(unsafe { (*p).p_out }, n_sp - 10,
                        ' ' as i32 as i8)
                };
                qrf_approx_int64(unsafe { (*p).p_out }, n_cycle_1);
                unsafe {
                    sqlite3_str_appendall(unsafe { (*p).p_out },
                        c" 100%\n".as_ptr() as *mut i8 as *const i8)
                };
            } else {
                unsafe {
                    sqlite3_str_appendall(unsafe { (*p).p_out },
                        c"QUERY PLAN\n".as_ptr() as *mut i8 as *const i8)
                };
            }
            unsafe {
                (*unsafe { (*p).u.p_graph }).z_prefix[0 as usize] = 0 as i8
            };
            qrf_eqp_render_level(p, 0);
            qrf_eqp_reset(unsafe { &mut *p });
        }
    }
}

///* Add a new entry to the EXPLAIN QUERY PLAN data
extern "C" fn qrf_eqp_append(p: *mut Qrf, i_eqp_id_1: i32, p2: i32,
    z_text_1: *const i8) -> () {
    unsafe {
        let mut p_new: *mut QrfEQPGraphRow = core::ptr::null_mut();
        let mut n_text: Sqlite3Int64 = 0 as Sqlite3Int64;
        if z_text_1 == core::ptr::null() { return; }
        if unsafe { (*p).u.p_graph } == core::ptr::null_mut() {
            unsafe {
                (*p).u.p_graph =
                    unsafe {
                            sqlite3_malloc64(core::mem::size_of::<QrfEQPGraph>() as
                                    Sqlite3Uint64)
                        } as *mut QrfEQPGraph
            };
            if unsafe { (*p).u.p_graph } == core::ptr::null_mut() {
                qrf_oom(p);
                return;
            }
            unsafe {
                memset(unsafe { (*p).u.p_graph } as *mut (), 0,
                    core::mem::size_of::<QrfEQPGraph>() as u64)
            };
        }
        n_text = unsafe { strlen(z_text_1) } as Sqlite3Int64;
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<QrfEQPGraphRow>() as
                                u64 + n_text as u64)
                } as *mut QrfEQPGraphRow;
        if p_new == core::ptr::null_mut() { qrf_oom(p); return; }
        unsafe { (*p_new).i_eqp_id = i_eqp_id_1 };
        unsafe { (*p_new).i_parent_id = p2 };
        unsafe {
            memcpy(unsafe { &raw mut (*p_new).z_text[0 as usize] } as *mut i8
                    as *mut (), z_text_1 as *const (),
                (n_text + 1 as Sqlite3Int64) as u64)
        };
        unsafe { (*p_new).p_next = core::ptr::null_mut() };
        if !(unsafe { (*unsafe { (*p).u.p_graph }).p_last }).is_null() {
            unsafe {
                (*unsafe { (*unsafe { (*p).u.p_graph }).p_last }).p_next =
                    p_new
            };
        } else { unsafe { (*unsafe { (*p).u.p_graph }).p_row = p_new }; }
        unsafe { (*unsafe { (*p).u.p_graph }).p_last = p_new };
    }
}

///* Render a single row of output for non-columnar styles - any
///* style that lets us render row by row as the content is received
///* from the query.
#[allow(unused_doc_comments)]
extern "C" fn qrf_one_simple_row(p: *mut Qrf) -> () {
    unsafe {
        let mut i: i32 = 0;
        '__s85:
            {
            match unsafe { (*p).spec.e_style } {
                14 => {
                    {

                        /// No-op
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"[{".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"},\n{".as_ptr() as *mut i8 as *const i8, 4)
                            };
                        }
                        qrf_one_json_row(p);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"{".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}\n{".as_ptr() as *mut i8 as *const i8, 3)
                            };
                        }
                        qrf_one_json_row(p);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                            };
                            {
                                i = 0;
                                '__b86: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b86; }
                                    '__c86: loop {
                                        let z_c_name: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n<TH>".as_ptr() as *mut i8 as *const i8, 5)
                                        };
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name);
                                        break '__c86;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                            };
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                        };
                        {
                            i = 0;
                            '__b87: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b87; }
                                '__c87: loop {
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            c"\n<TD>".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c87;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mx_ins: u32 = unsafe { (*p).spec.n_multi_insert };
                        let sz_start: i32 =
                            unsafe { sqlite3_str_length(unsafe { (*p).p_out }) };
                        if unsafe { (*p).u.n_ins } == 0 as u32 ||
                                unsafe { (*p).u.n_ins } >= mx_ins {
                            if unsafe { (*p).u.n_ins } != 0 {
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c";\n".as_ptr() as *mut i8 as *const i8, 2)
                                };
                                unsafe { (*p).u.n_ins = 0 as u32 };
                            }
                            if qrf_need_quote(unsafe { (*p).spec.z_table_name } as
                                            *const i8) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO \"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            } else {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO %s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            }
                            if unsafe { (*p).spec.b_titles } as i32 == 2 {
                                {
                                    i = 0;
                                    '__b88: loop {
                                        if !(i < unsafe { (*p).n_col }) { break '__b88; }
                                        '__c88: loop {
                                            let z_c_name_1: *const i8 =
                                                unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                            if qrf_need_quote(z_c_name_1) != 0 {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c\"%w\"".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            } else {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c%s".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            }
                                            break '__c88;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c" VALUES(".as_ptr() as *mut i8 as *const i8, 8)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c",\n  (".as_ptr() as *mut i8 as *const i8, 5)
                            };
                        }
                        {
                            i = 0;
                            '__b89: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b89; }
                                '__c89: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c",".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c89;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            (*p).u.n_ins +=
                                (unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } + 2 -
                                        sz_start) as u32
                        };
                        if unsafe { (*p).u.n_ins } >= mx_ins {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c");\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                            unsafe { (*p).u.n_ins = 0 as u32 };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c")".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut p_val: *mut Sqlite3Str = core::ptr::null_mut();
                        let mut mx_w: i32 = 0;
                        let mut b_ww: i32 = 0;
                        let mut n_sep: i32 = 0;
                        if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                            {
                            unsafe {
                                (*p).u.s_line.az_col =
                                    unsafe {
                                            sqlite3_malloc64(unsafe { (*p).n_col } as u64 *
                                                    core::mem::size_of::<*mut i8>() as u64)
                                        } as *mut *mut i8
                            };
                            if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                                {
                                qrf_oom(p);
                                break '__s85;
                            }
                            unsafe { (*p).u.s_line.mx_col_wth = 0 };
                            {
                                i = 0;
                                '__b90: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b90; }
                                    '__c90: loop {
                                        let mut sz: i32 = 0;
                                        let mut z_c_name_2: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if z_c_name_2 == core::ptr::null() {
                                            z_c_name_2 = c"unknown".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) } =
                                                unsafe {
                                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        z_c_name_2)
                                                }
                                        };
                                        if unsafe { (*p).spec.n_title_limit } as i32 > 0 {
                                            {
                                                let _ =
                                                    qrf_title_limit(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        }, unsafe { (*p).spec.n_title_limit } as i32);
                                            };
                                        }
                                        sz =
                                            sqlite3_qrf_wcswidth(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        } as *const i8) as i32;
                                        if sz > unsafe { (*p).u.s_line.mx_col_wth } {
                                            unsafe { (*p).u.s_line.mx_col_wth = sz };
                                        }
                                        break '__c90;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p).n_row } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        p_val = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        n_sep =
                            unsafe {
                                    strlen(unsafe { (*p).spec.z_column_sep } as *const i8)
                                } as i32;
                        mx_w =
                            unsafe { (*p).mx_width } -
                                (n_sep + unsafe { (*p).u.s_line.mx_col_wth });
                        b_ww =
                            (unsafe { (*p).spec.b_word_wrap } as i32 == 2) as i32;
                        {
                            i = 0;
                            '__b91: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b91; }
                                '__c91: loop {
                                    let mut z_val: *const i8 = core::ptr::null();
                                    let mut cnt: i32 = 0;
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        -unsafe { (*p).u.s_line.mx_col_wth },
                                        unsafe {
                                                *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                            } as *const i8);
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            unsafe { (*p).spec.z_column_sep } as *const i8, n_sep)
                                    };
                                    qrf_render_value(p, p_val, i);
                                    z_val = unsafe { sqlite3_str_value(p_val) } as *const i8;
                                    if z_val == core::ptr::null() {
                                        z_val = c"".as_ptr() as *mut i8 as *const i8;
                                    }
                                    '__b92: loop {
                                        '__c92: loop {
                                            let mut n_this: i32 = 0;
                                            let mut n_wide: i32 = 0;
                                            let mut i_next: i32 = 0;
                                            qrf_wrap_line(z_val, mx_w, b_ww, &mut n_this, &mut n_wide,
                                                &mut i_next);
                                            if cnt != 0 {
                                                unsafe {
                                                    sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                        unsafe { (*p).u.s_line.mx_col_wth } + n_sep,
                                                        ' ' as i32 as i8)
                                                };
                                            }
                                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                                            if cnt > unsafe { (*p).mx_height } {
                                                z_val = c"...".as_ptr() as *mut i8 as *const i8;
                                                n_this = { i_next = 3; i_next };
                                            }
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out }, z_val, n_this)
                                            };
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out },
                                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                            };
                                            {
                                                let __n = i_next;
                                                let __p = &mut z_val;
                                                *__p = unsafe { (*__p).offset(__n as isize) };
                                            };
                                            break '__c92;
                                        }
                                        if !(unsafe { *z_val.offset(0 as isize) } != 0) {
                                            break '__b92;
                                        }
                                    }
                                    unsafe { sqlite3_str_reset(p_val) };
                                    break '__c91;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        qrf_str_err(p, p_val);
                        unsafe {
                            sqlite3_free(unsafe { sqlite3_str_finish(p_val) } as
                                    *mut ())
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                3 => {
                    {

                        /// No-op
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"[{".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"},\n{".as_ptr() as *mut i8 as *const i8, 4)
                            };
                        }
                        qrf_one_json_row(p);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"{".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}\n{".as_ptr() as *mut i8 as *const i8, 3)
                            };
                        }
                        qrf_one_json_row(p);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                            };
                            {
                                i = 0;
                                '__b86: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b86; }
                                    '__c86: loop {
                                        let z_c_name: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n<TH>".as_ptr() as *mut i8 as *const i8, 5)
                                        };
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name);
                                        break '__c86;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                            };
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                        };
                        {
                            i = 0;
                            '__b87: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b87; }
                                '__c87: loop {
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            c"\n<TD>".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c87;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mx_ins: u32 = unsafe { (*p).spec.n_multi_insert };
                        let sz_start: i32 =
                            unsafe { sqlite3_str_length(unsafe { (*p).p_out }) };
                        if unsafe { (*p).u.n_ins } == 0 as u32 ||
                                unsafe { (*p).u.n_ins } >= mx_ins {
                            if unsafe { (*p).u.n_ins } != 0 {
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c";\n".as_ptr() as *mut i8 as *const i8, 2)
                                };
                                unsafe { (*p).u.n_ins = 0 as u32 };
                            }
                            if qrf_need_quote(unsafe { (*p).spec.z_table_name } as
                                            *const i8) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO \"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            } else {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO %s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            }
                            if unsafe { (*p).spec.b_titles } as i32 == 2 {
                                {
                                    i = 0;
                                    '__b88: loop {
                                        if !(i < unsafe { (*p).n_col }) { break '__b88; }
                                        '__c88: loop {
                                            let z_c_name_1: *const i8 =
                                                unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                            if qrf_need_quote(z_c_name_1) != 0 {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c\"%w\"".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            } else {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c%s".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            }
                                            break '__c88;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c" VALUES(".as_ptr() as *mut i8 as *const i8, 8)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c",\n  (".as_ptr() as *mut i8 as *const i8, 5)
                            };
                        }
                        {
                            i = 0;
                            '__b89: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b89; }
                                '__c89: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c",".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c89;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            (*p).u.n_ins +=
                                (unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } + 2 -
                                        sz_start) as u32
                        };
                        if unsafe { (*p).u.n_ins } >= mx_ins {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c");\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                            unsafe { (*p).u.n_ins = 0 as u32 };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c")".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut p_val: *mut Sqlite3Str = core::ptr::null_mut();
                        let mut mx_w: i32 = 0;
                        let mut b_ww: i32 = 0;
                        let mut n_sep: i32 = 0;
                        if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                            {
                            unsafe {
                                (*p).u.s_line.az_col =
                                    unsafe {
                                            sqlite3_malloc64(unsafe { (*p).n_col } as u64 *
                                                    core::mem::size_of::<*mut i8>() as u64)
                                        } as *mut *mut i8
                            };
                            if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                                {
                                qrf_oom(p);
                                break '__s85;
                            }
                            unsafe { (*p).u.s_line.mx_col_wth = 0 };
                            {
                                i = 0;
                                '__b90: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b90; }
                                    '__c90: loop {
                                        let mut sz: i32 = 0;
                                        let mut z_c_name_2: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if z_c_name_2 == core::ptr::null() {
                                            z_c_name_2 = c"unknown".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) } =
                                                unsafe {
                                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        z_c_name_2)
                                                }
                                        };
                                        if unsafe { (*p).spec.n_title_limit } as i32 > 0 {
                                            {
                                                let _ =
                                                    qrf_title_limit(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        }, unsafe { (*p).spec.n_title_limit } as i32);
                                            };
                                        }
                                        sz =
                                            sqlite3_qrf_wcswidth(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        } as *const i8) as i32;
                                        if sz > unsafe { (*p).u.s_line.mx_col_wth } {
                                            unsafe { (*p).u.s_line.mx_col_wth = sz };
                                        }
                                        break '__c90;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p).n_row } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        p_val = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        n_sep =
                            unsafe {
                                    strlen(unsafe { (*p).spec.z_column_sep } as *const i8)
                                } as i32;
                        mx_w =
                            unsafe { (*p).mx_width } -
                                (n_sep + unsafe { (*p).u.s_line.mx_col_wth });
                        b_ww =
                            (unsafe { (*p).spec.b_word_wrap } as i32 == 2) as i32;
                        {
                            i = 0;
                            '__b91: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b91; }
                                '__c91: loop {
                                    let mut z_val: *const i8 = core::ptr::null();
                                    let mut cnt: i32 = 0;
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        -unsafe { (*p).u.s_line.mx_col_wth },
                                        unsafe {
                                                *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                            } as *const i8);
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            unsafe { (*p).spec.z_column_sep } as *const i8, n_sep)
                                    };
                                    qrf_render_value(p, p_val, i);
                                    z_val = unsafe { sqlite3_str_value(p_val) } as *const i8;
                                    if z_val == core::ptr::null() {
                                        z_val = c"".as_ptr() as *mut i8 as *const i8;
                                    }
                                    '__b92: loop {
                                        '__c92: loop {
                                            let mut n_this: i32 = 0;
                                            let mut n_wide: i32 = 0;
                                            let mut i_next: i32 = 0;
                                            qrf_wrap_line(z_val, mx_w, b_ww, &mut n_this, &mut n_wide,
                                                &mut i_next);
                                            if cnt != 0 {
                                                unsafe {
                                                    sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                        unsafe { (*p).u.s_line.mx_col_wth } + n_sep,
                                                        ' ' as i32 as i8)
                                                };
                                            }
                                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                                            if cnt > unsafe { (*p).mx_height } {
                                                z_val = c"...".as_ptr() as *mut i8 as *const i8;
                                                n_this = { i_next = 3; i_next };
                                            }
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out }, z_val, n_this)
                                            };
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out },
                                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                            };
                                            {
                                                let __n = i_next;
                                                let __p = &mut z_val;
                                                *__p = unsafe { (*__p).offset(__n as isize) };
                                            };
                                            break '__c92;
                                        }
                                        if !(unsafe { *z_val.offset(0 as isize) } != 0) {
                                            break '__b92;
                                        }
                                    }
                                    unsafe { sqlite3_str_reset(p_val) };
                                    break '__c91;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        qrf_str_err(p, p_val);
                        unsafe {
                            sqlite3_free(unsafe { sqlite3_str_finish(p_val) } as
                                    *mut ())
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                9 => {
                    {
                        if unsafe { (*p).n_row } == 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"[{".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"},\n{".as_ptr() as *mut i8 as *const i8, 4)
                            };
                        }
                        qrf_one_json_row(p);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"{".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}\n{".as_ptr() as *mut i8 as *const i8, 3)
                            };
                        }
                        qrf_one_json_row(p);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                            };
                            {
                                i = 0;
                                '__b86: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b86; }
                                    '__c86: loop {
                                        let z_c_name: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n<TH>".as_ptr() as *mut i8 as *const i8, 5)
                                        };
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name);
                                        break '__c86;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                            };
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                        };
                        {
                            i = 0;
                            '__b87: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b87; }
                                '__c87: loop {
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            c"\n<TD>".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c87;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mx_ins: u32 = unsafe { (*p).spec.n_multi_insert };
                        let sz_start: i32 =
                            unsafe { sqlite3_str_length(unsafe { (*p).p_out }) };
                        if unsafe { (*p).u.n_ins } == 0 as u32 ||
                                unsafe { (*p).u.n_ins } >= mx_ins {
                            if unsafe { (*p).u.n_ins } != 0 {
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c";\n".as_ptr() as *mut i8 as *const i8, 2)
                                };
                                unsafe { (*p).u.n_ins = 0 as u32 };
                            }
                            if qrf_need_quote(unsafe { (*p).spec.z_table_name } as
                                            *const i8) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO \"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            } else {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO %s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            }
                            if unsafe { (*p).spec.b_titles } as i32 == 2 {
                                {
                                    i = 0;
                                    '__b88: loop {
                                        if !(i < unsafe { (*p).n_col }) { break '__b88; }
                                        '__c88: loop {
                                            let z_c_name_1: *const i8 =
                                                unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                            if qrf_need_quote(z_c_name_1) != 0 {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c\"%w\"".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            } else {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c%s".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            }
                                            break '__c88;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c" VALUES(".as_ptr() as *mut i8 as *const i8, 8)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c",\n  (".as_ptr() as *mut i8 as *const i8, 5)
                            };
                        }
                        {
                            i = 0;
                            '__b89: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b89; }
                                '__c89: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c",".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c89;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            (*p).u.n_ins +=
                                (unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } + 2 -
                                        sz_start) as u32
                        };
                        if unsafe { (*p).u.n_ins } >= mx_ins {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c");\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                            unsafe { (*p).u.n_ins = 0 as u32 };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c")".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut p_val: *mut Sqlite3Str = core::ptr::null_mut();
                        let mut mx_w: i32 = 0;
                        let mut b_ww: i32 = 0;
                        let mut n_sep: i32 = 0;
                        if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                            {
                            unsafe {
                                (*p).u.s_line.az_col =
                                    unsafe {
                                            sqlite3_malloc64(unsafe { (*p).n_col } as u64 *
                                                    core::mem::size_of::<*mut i8>() as u64)
                                        } as *mut *mut i8
                            };
                            if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                                {
                                qrf_oom(p);
                                break '__s85;
                            }
                            unsafe { (*p).u.s_line.mx_col_wth = 0 };
                            {
                                i = 0;
                                '__b90: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b90; }
                                    '__c90: loop {
                                        let mut sz: i32 = 0;
                                        let mut z_c_name_2: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if z_c_name_2 == core::ptr::null() {
                                            z_c_name_2 = c"unknown".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) } =
                                                unsafe {
                                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        z_c_name_2)
                                                }
                                        };
                                        if unsafe { (*p).spec.n_title_limit } as i32 > 0 {
                                            {
                                                let _ =
                                                    qrf_title_limit(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        }, unsafe { (*p).spec.n_title_limit } as i32);
                                            };
                                        }
                                        sz =
                                            sqlite3_qrf_wcswidth(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        } as *const i8) as i32;
                                        if sz > unsafe { (*p).u.s_line.mx_col_wth } {
                                            unsafe { (*p).u.s_line.mx_col_wth = sz };
                                        }
                                        break '__c90;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p).n_row } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        p_val = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        n_sep =
                            unsafe {
                                    strlen(unsafe { (*p).spec.z_column_sep } as *const i8)
                                } as i32;
                        mx_w =
                            unsafe { (*p).mx_width } -
                                (n_sep + unsafe { (*p).u.s_line.mx_col_wth });
                        b_ww =
                            (unsafe { (*p).spec.b_word_wrap } as i32 == 2) as i32;
                        {
                            i = 0;
                            '__b91: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b91; }
                                '__c91: loop {
                                    let mut z_val: *const i8 = core::ptr::null();
                                    let mut cnt: i32 = 0;
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        -unsafe { (*p).u.s_line.mx_col_wth },
                                        unsafe {
                                                *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                            } as *const i8);
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            unsafe { (*p).spec.z_column_sep } as *const i8, n_sep)
                                    };
                                    qrf_render_value(p, p_val, i);
                                    z_val = unsafe { sqlite3_str_value(p_val) } as *const i8;
                                    if z_val == core::ptr::null() {
                                        z_val = c"".as_ptr() as *mut i8 as *const i8;
                                    }
                                    '__b92: loop {
                                        '__c92: loop {
                                            let mut n_this: i32 = 0;
                                            let mut n_wide: i32 = 0;
                                            let mut i_next: i32 = 0;
                                            qrf_wrap_line(z_val, mx_w, b_ww, &mut n_this, &mut n_wide,
                                                &mut i_next);
                                            if cnt != 0 {
                                                unsafe {
                                                    sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                        unsafe { (*p).u.s_line.mx_col_wth } + n_sep,
                                                        ' ' as i32 as i8)
                                                };
                                            }
                                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                                            if cnt > unsafe { (*p).mx_height } {
                                                z_val = c"...".as_ptr() as *mut i8 as *const i8;
                                                n_this = { i_next = 3; i_next };
                                            }
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out }, z_val, n_this)
                                            };
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out },
                                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                            };
                                            {
                                                let __n = i_next;
                                                let __p = &mut z_val;
                                                *__p = unsafe { (*__p).offset(__n as isize) };
                                            };
                                            break '__c92;
                                        }
                                        if !(unsafe { *z_val.offset(0 as isize) } != 0) {
                                            break '__b92;
                                        }
                                    }
                                    unsafe { sqlite3_str_reset(p_val) };
                                    break '__c91;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        qrf_str_err(p, p_val);
                        unsafe {
                            sqlite3_free(unsafe { sqlite3_str_finish(p_val) } as
                                    *mut ())
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                10 => {
                    {
                        if unsafe { (*p).n_row } == 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"{".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}\n{".as_ptr() as *mut i8 as *const i8, 3)
                            };
                        }
                        qrf_one_json_row(p);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                            };
                            {
                                i = 0;
                                '__b86: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b86; }
                                    '__c86: loop {
                                        let z_c_name: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n<TH>".as_ptr() as *mut i8 as *const i8, 5)
                                        };
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name);
                                        break '__c86;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                            };
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                        };
                        {
                            i = 0;
                            '__b87: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b87; }
                                '__c87: loop {
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            c"\n<TD>".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c87;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mx_ins: u32 = unsafe { (*p).spec.n_multi_insert };
                        let sz_start: i32 =
                            unsafe { sqlite3_str_length(unsafe { (*p).p_out }) };
                        if unsafe { (*p).u.n_ins } == 0 as u32 ||
                                unsafe { (*p).u.n_ins } >= mx_ins {
                            if unsafe { (*p).u.n_ins } != 0 {
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c";\n".as_ptr() as *mut i8 as *const i8, 2)
                                };
                                unsafe { (*p).u.n_ins = 0 as u32 };
                            }
                            if qrf_need_quote(unsafe { (*p).spec.z_table_name } as
                                            *const i8) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO \"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            } else {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO %s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            }
                            if unsafe { (*p).spec.b_titles } as i32 == 2 {
                                {
                                    i = 0;
                                    '__b88: loop {
                                        if !(i < unsafe { (*p).n_col }) { break '__b88; }
                                        '__c88: loop {
                                            let z_c_name_1: *const i8 =
                                                unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                            if qrf_need_quote(z_c_name_1) != 0 {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c\"%w\"".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            } else {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c%s".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            }
                                            break '__c88;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c" VALUES(".as_ptr() as *mut i8 as *const i8, 8)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c",\n  (".as_ptr() as *mut i8 as *const i8, 5)
                            };
                        }
                        {
                            i = 0;
                            '__b89: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b89; }
                                '__c89: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c",".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c89;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            (*p).u.n_ins +=
                                (unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } + 2 -
                                        sz_start) as u32
                        };
                        if unsafe { (*p).u.n_ins } >= mx_ins {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c");\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                            unsafe { (*p).u.n_ins = 0 as u32 };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c")".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut p_val: *mut Sqlite3Str = core::ptr::null_mut();
                        let mut mx_w: i32 = 0;
                        let mut b_ww: i32 = 0;
                        let mut n_sep: i32 = 0;
                        if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                            {
                            unsafe {
                                (*p).u.s_line.az_col =
                                    unsafe {
                                            sqlite3_malloc64(unsafe { (*p).n_col } as u64 *
                                                    core::mem::size_of::<*mut i8>() as u64)
                                        } as *mut *mut i8
                            };
                            if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                                {
                                qrf_oom(p);
                                break '__s85;
                            }
                            unsafe { (*p).u.s_line.mx_col_wth = 0 };
                            {
                                i = 0;
                                '__b90: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b90; }
                                    '__c90: loop {
                                        let mut sz: i32 = 0;
                                        let mut z_c_name_2: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if z_c_name_2 == core::ptr::null() {
                                            z_c_name_2 = c"unknown".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) } =
                                                unsafe {
                                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        z_c_name_2)
                                                }
                                        };
                                        if unsafe { (*p).spec.n_title_limit } as i32 > 0 {
                                            {
                                                let _ =
                                                    qrf_title_limit(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        }, unsafe { (*p).spec.n_title_limit } as i32);
                                            };
                                        }
                                        sz =
                                            sqlite3_qrf_wcswidth(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        } as *const i8) as i32;
                                        if sz > unsafe { (*p).u.s_line.mx_col_wth } {
                                            unsafe { (*p).u.s_line.mx_col_wth = sz };
                                        }
                                        break '__c90;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p).n_row } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        p_val = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        n_sep =
                            unsafe {
                                    strlen(unsafe { (*p).spec.z_column_sep } as *const i8)
                                } as i32;
                        mx_w =
                            unsafe { (*p).mx_width } -
                                (n_sep + unsafe { (*p).u.s_line.mx_col_wth });
                        b_ww =
                            (unsafe { (*p).spec.b_word_wrap } as i32 == 2) as i32;
                        {
                            i = 0;
                            '__b91: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b91; }
                                '__c91: loop {
                                    let mut z_val: *const i8 = core::ptr::null();
                                    let mut cnt: i32 = 0;
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        -unsafe { (*p).u.s_line.mx_col_wth },
                                        unsafe {
                                                *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                            } as *const i8);
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            unsafe { (*p).spec.z_column_sep } as *const i8, n_sep)
                                    };
                                    qrf_render_value(p, p_val, i);
                                    z_val = unsafe { sqlite3_str_value(p_val) } as *const i8;
                                    if z_val == core::ptr::null() {
                                        z_val = c"".as_ptr() as *mut i8 as *const i8;
                                    }
                                    '__b92: loop {
                                        '__c92: loop {
                                            let mut n_this: i32 = 0;
                                            let mut n_wide: i32 = 0;
                                            let mut i_next: i32 = 0;
                                            qrf_wrap_line(z_val, mx_w, b_ww, &mut n_this, &mut n_wide,
                                                &mut i_next);
                                            if cnt != 0 {
                                                unsafe {
                                                    sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                        unsafe { (*p).u.s_line.mx_col_wth } + n_sep,
                                                        ' ' as i32 as i8)
                                                };
                                            }
                                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                                            if cnt > unsafe { (*p).mx_height } {
                                                z_val = c"...".as_ptr() as *mut i8 as *const i8;
                                                n_this = { i_next = 3; i_next };
                                            }
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out }, z_val, n_this)
                                            };
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out },
                                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                            };
                                            {
                                                let __n = i_next;
                                                let __p = &mut z_val;
                                                *__p = unsafe { (*__p).offset(__n as isize) };
                                            };
                                            break '__c92;
                                        }
                                        if !(unsafe { *z_val.offset(0 as isize) } != 0) {
                                            break '__b92;
                                        }
                                    }
                                    unsafe { sqlite3_str_reset(p_val) };
                                    break '__c91;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        qrf_str_err(p, p_val);
                        unsafe {
                            sqlite3_free(unsafe { sqlite3_str_finish(p_val) } as
                                    *mut ())
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                7 => {
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                            };
                            {
                                i = 0;
                                '__b86: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b86; }
                                    '__c86: loop {
                                        let z_c_name: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c"\n<TH>".as_ptr() as *mut i8 as *const i8, 5)
                                        };
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name);
                                        break '__c86;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                            };
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"<TR>".as_ptr() as *mut i8 as *const i8, 4)
                        };
                        {
                            i = 0;
                            '__b87: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b87; }
                                '__c87: loop {
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            c"\n<TD>".as_ptr() as *mut i8 as *const i8, 5)
                                    };
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c87;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_append(unsafe { (*p).p_out },
                                c"\n</TR>\n".as_ptr() as *mut i8 as *const i8, 7)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mx_ins: u32 = unsafe { (*p).spec.n_multi_insert };
                        let sz_start: i32 =
                            unsafe { sqlite3_str_length(unsafe { (*p).p_out }) };
                        if unsafe { (*p).u.n_ins } == 0 as u32 ||
                                unsafe { (*p).u.n_ins } >= mx_ins {
                            if unsafe { (*p).u.n_ins } != 0 {
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c";\n".as_ptr() as *mut i8 as *const i8, 2)
                                };
                                unsafe { (*p).u.n_ins = 0 as u32 };
                            }
                            if qrf_need_quote(unsafe { (*p).spec.z_table_name } as
                                            *const i8) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO \"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            } else {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO %s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            }
                            if unsafe { (*p).spec.b_titles } as i32 == 2 {
                                {
                                    i = 0;
                                    '__b88: loop {
                                        if !(i < unsafe { (*p).n_col }) { break '__b88; }
                                        '__c88: loop {
                                            let z_c_name_1: *const i8 =
                                                unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                            if qrf_need_quote(z_c_name_1) != 0 {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c\"%w\"".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            } else {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c%s".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            }
                                            break '__c88;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c" VALUES(".as_ptr() as *mut i8 as *const i8, 8)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c",\n  (".as_ptr() as *mut i8 as *const i8, 5)
                            };
                        }
                        {
                            i = 0;
                            '__b89: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b89; }
                                '__c89: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c",".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c89;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            (*p).u.n_ins +=
                                (unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } + 2 -
                                        sz_start) as u32
                        };
                        if unsafe { (*p).u.n_ins } >= mx_ins {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c");\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                            unsafe { (*p).u.n_ins = 0 as u32 };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c")".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut p_val: *mut Sqlite3Str = core::ptr::null_mut();
                        let mut mx_w: i32 = 0;
                        let mut b_ww: i32 = 0;
                        let mut n_sep: i32 = 0;
                        if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                            {
                            unsafe {
                                (*p).u.s_line.az_col =
                                    unsafe {
                                            sqlite3_malloc64(unsafe { (*p).n_col } as u64 *
                                                    core::mem::size_of::<*mut i8>() as u64)
                                        } as *mut *mut i8
                            };
                            if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                                {
                                qrf_oom(p);
                                break '__s85;
                            }
                            unsafe { (*p).u.s_line.mx_col_wth = 0 };
                            {
                                i = 0;
                                '__b90: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b90; }
                                    '__c90: loop {
                                        let mut sz: i32 = 0;
                                        let mut z_c_name_2: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if z_c_name_2 == core::ptr::null() {
                                            z_c_name_2 = c"unknown".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) } =
                                                unsafe {
                                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        z_c_name_2)
                                                }
                                        };
                                        if unsafe { (*p).spec.n_title_limit } as i32 > 0 {
                                            {
                                                let _ =
                                                    qrf_title_limit(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        }, unsafe { (*p).spec.n_title_limit } as i32);
                                            };
                                        }
                                        sz =
                                            sqlite3_qrf_wcswidth(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        } as *const i8) as i32;
                                        if sz > unsafe { (*p).u.s_line.mx_col_wth } {
                                            unsafe { (*p).u.s_line.mx_col_wth = sz };
                                        }
                                        break '__c90;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p).n_row } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        p_val = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        n_sep =
                            unsafe {
                                    strlen(unsafe { (*p).spec.z_column_sep } as *const i8)
                                } as i32;
                        mx_w =
                            unsafe { (*p).mx_width } -
                                (n_sep + unsafe { (*p).u.s_line.mx_col_wth });
                        b_ww =
                            (unsafe { (*p).spec.b_word_wrap } as i32 == 2) as i32;
                        {
                            i = 0;
                            '__b91: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b91; }
                                '__c91: loop {
                                    let mut z_val: *const i8 = core::ptr::null();
                                    let mut cnt: i32 = 0;
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        -unsafe { (*p).u.s_line.mx_col_wth },
                                        unsafe {
                                                *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                            } as *const i8);
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            unsafe { (*p).spec.z_column_sep } as *const i8, n_sep)
                                    };
                                    qrf_render_value(p, p_val, i);
                                    z_val = unsafe { sqlite3_str_value(p_val) } as *const i8;
                                    if z_val == core::ptr::null() {
                                        z_val = c"".as_ptr() as *mut i8 as *const i8;
                                    }
                                    '__b92: loop {
                                        '__c92: loop {
                                            let mut n_this: i32 = 0;
                                            let mut n_wide: i32 = 0;
                                            let mut i_next: i32 = 0;
                                            qrf_wrap_line(z_val, mx_w, b_ww, &mut n_this, &mut n_wide,
                                                &mut i_next);
                                            if cnt != 0 {
                                                unsafe {
                                                    sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                        unsafe { (*p).u.s_line.mx_col_wth } + n_sep,
                                                        ' ' as i32 as i8)
                                                };
                                            }
                                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                                            if cnt > unsafe { (*p).mx_height } {
                                                z_val = c"...".as_ptr() as *mut i8 as *const i8;
                                                n_this = { i_next = 3; i_next };
                                            }
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out }, z_val, n_this)
                                            };
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out },
                                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                            };
                                            {
                                                let __n = i_next;
                                                let __p = &mut z_val;
                                                *__p = unsafe { (*__p).offset(__n as isize) };
                                            };
                                            break '__c92;
                                        }
                                        if !(unsafe { *z_val.offset(0 as isize) } != 0) {
                                            break '__b92;
                                        }
                                    }
                                    unsafe { sqlite3_str_reset(p_val) };
                                    break '__c91;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        qrf_str_err(p, p_val);
                        unsafe {
                            sqlite3_free(unsafe { sqlite3_str_finish(p_val) } as
                                    *mut ())
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                8 => {
                    {
                        let mx_ins: u32 = unsafe { (*p).spec.n_multi_insert };
                        let sz_start: i32 =
                            unsafe { sqlite3_str_length(unsafe { (*p).p_out }) };
                        if unsafe { (*p).u.n_ins } == 0 as u32 ||
                                unsafe { (*p).u.n_ins } >= mx_ins {
                            if unsafe { (*p).u.n_ins } != 0 {
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c";\n".as_ptr() as *mut i8 as *const i8, 2)
                                };
                                unsafe { (*p).u.n_ins = 0 as u32 };
                            }
                            if qrf_need_quote(unsafe { (*p).spec.z_table_name } as
                                            *const i8) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO \"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            } else {
                                unsafe {
                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                        c"INSERT INTO %s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).spec.z_table_name })
                                };
                            }
                            if unsafe { (*p).spec.b_titles } as i32 == 2 {
                                {
                                    i = 0;
                                    '__b88: loop {
                                        if !(i < unsafe { (*p).n_col }) { break '__b88; }
                                        '__c88: loop {
                                            let z_c_name_1: *const i8 =
                                                unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                            if qrf_need_quote(z_c_name_1) != 0 {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c\"%w\"".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            } else {
                                                unsafe {
                                                    sqlite3_str_appendf(unsafe { (*p).p_out },
                                                        c"%c%s".as_ptr() as *mut i8 as *const i8,
                                                        if i == 0 { '(' as i32 } else { ',' as i32 }, z_c_name_1)
                                                };
                                            }
                                            break '__c88;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    sqlite3_str_append(unsafe { (*p).p_out },
                                        c")".as_ptr() as *mut i8 as *const i8, 1)
                                };
                            }
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c" VALUES(".as_ptr() as *mut i8 as *const i8, 8)
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c",\n  (".as_ptr() as *mut i8 as *const i8, 5)
                            };
                        }
                        {
                            i = 0;
                            '__b89: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b89; }
                                '__c89: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_append(unsafe { (*p).p_out },
                                                c",".as_ptr() as *mut i8 as *const i8, 1)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c89;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            (*p).u.n_ins +=
                                (unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } + 2 -
                                        sz_start) as u32
                        };
                        if unsafe { (*p).u.n_ins } >= mx_ins {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c");\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                            unsafe { (*p).u.n_ins = 0 as u32 };
                        } else {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c")".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut p_val: *mut Sqlite3Str = core::ptr::null_mut();
                        let mut mx_w: i32 = 0;
                        let mut b_ww: i32 = 0;
                        let mut n_sep: i32 = 0;
                        if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                            {
                            unsafe {
                                (*p).u.s_line.az_col =
                                    unsafe {
                                            sqlite3_malloc64(unsafe { (*p).n_col } as u64 *
                                                    core::mem::size_of::<*mut i8>() as u64)
                                        } as *mut *mut i8
                            };
                            if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                                {
                                qrf_oom(p);
                                break '__s85;
                            }
                            unsafe { (*p).u.s_line.mx_col_wth = 0 };
                            {
                                i = 0;
                                '__b90: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b90; }
                                    '__c90: loop {
                                        let mut sz: i32 = 0;
                                        let mut z_c_name_2: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if z_c_name_2 == core::ptr::null() {
                                            z_c_name_2 = c"unknown".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) } =
                                                unsafe {
                                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        z_c_name_2)
                                                }
                                        };
                                        if unsafe { (*p).spec.n_title_limit } as i32 > 0 {
                                            {
                                                let _ =
                                                    qrf_title_limit(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        }, unsafe { (*p).spec.n_title_limit } as i32);
                                            };
                                        }
                                        sz =
                                            sqlite3_qrf_wcswidth(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        } as *const i8) as i32;
                                        if sz > unsafe { (*p).u.s_line.mx_col_wth } {
                                            unsafe { (*p).u.s_line.mx_col_wth = sz };
                                        }
                                        break '__c90;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p).n_row } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        p_val = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        n_sep =
                            unsafe {
                                    strlen(unsafe { (*p).spec.z_column_sep } as *const i8)
                                } as i32;
                        mx_w =
                            unsafe { (*p).mx_width } -
                                (n_sep + unsafe { (*p).u.s_line.mx_col_wth });
                        b_ww =
                            (unsafe { (*p).spec.b_word_wrap } as i32 == 2) as i32;
                        {
                            i = 0;
                            '__b91: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b91; }
                                '__c91: loop {
                                    let mut z_val: *const i8 = core::ptr::null();
                                    let mut cnt: i32 = 0;
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        -unsafe { (*p).u.s_line.mx_col_wth },
                                        unsafe {
                                                *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                            } as *const i8);
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            unsafe { (*p).spec.z_column_sep } as *const i8, n_sep)
                                    };
                                    qrf_render_value(p, p_val, i);
                                    z_val = unsafe { sqlite3_str_value(p_val) } as *const i8;
                                    if z_val == core::ptr::null() {
                                        z_val = c"".as_ptr() as *mut i8 as *const i8;
                                    }
                                    '__b92: loop {
                                        '__c92: loop {
                                            let mut n_this: i32 = 0;
                                            let mut n_wide: i32 = 0;
                                            let mut i_next: i32 = 0;
                                            qrf_wrap_line(z_val, mx_w, b_ww, &mut n_this, &mut n_wide,
                                                &mut i_next);
                                            if cnt != 0 {
                                                unsafe {
                                                    sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                        unsafe { (*p).u.s_line.mx_col_wth } + n_sep,
                                                        ' ' as i32 as i8)
                                                };
                                            }
                                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                                            if cnt > unsafe { (*p).mx_height } {
                                                z_val = c"...".as_ptr() as *mut i8 as *const i8;
                                                n_this = { i_next = 3; i_next };
                                            }
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out }, z_val, n_this)
                                            };
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out },
                                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                            };
                                            {
                                                let __n = i_next;
                                                let __p = &mut z_val;
                                                *__p = unsafe { (*__p).offset(__n as isize) };
                                            };
                                            break '__c92;
                                        }
                                        if !(unsafe { *z_val.offset(0 as isize) } != 0) {
                                            break '__b92;
                                        }
                                    }
                                    unsafe { sqlite3_str_reset(p_val) };
                                    break '__c91;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        qrf_str_err(p, p_val);
                        unsafe {
                            sqlite3_free(unsafe { sqlite3_str_finish(p_val) } as
                                    *mut ())
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                11 => {
                    {
                        let mut p_val: *mut Sqlite3Str = core::ptr::null_mut();
                        let mut mx_w: i32 = 0;
                        let mut b_ww: i32 = 0;
                        let mut n_sep: i32 = 0;
                        if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                            {
                            unsafe {
                                (*p).u.s_line.az_col =
                                    unsafe {
                                            sqlite3_malloc64(unsafe { (*p).n_col } as u64 *
                                                    core::mem::size_of::<*mut i8>() as u64)
                                        } as *mut *mut i8
                            };
                            if unsafe { (*p).u.s_line.az_col } == core::ptr::null_mut()
                                {
                                qrf_oom(p);
                                break '__s85;
                            }
                            unsafe { (*p).u.s_line.mx_col_wth = 0 };
                            {
                                i = 0;
                                '__b90: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b90; }
                                    '__c90: loop {
                                        let mut sz: i32 = 0;
                                        let mut z_c_name_2: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if z_c_name_2 == core::ptr::null() {
                                            z_c_name_2 = c"unknown".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) } =
                                                unsafe {
                                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        z_c_name_2)
                                                }
                                        };
                                        if unsafe { (*p).spec.n_title_limit } as i32 > 0 {
                                            {
                                                let _ =
                                                    qrf_title_limit(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        }, unsafe { (*p).spec.n_title_limit } as i32);
                                            };
                                        }
                                        sz =
                                            sqlite3_qrf_wcswidth(unsafe {
                                                            *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                        } as *const i8) as i32;
                                        if sz > unsafe { (*p).u.s_line.mx_col_wth } {
                                            unsafe { (*p).u.s_line.mx_col_wth = sz };
                                        }
                                        break '__c90;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p).n_row } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                            };
                        }
                        p_val = unsafe { sqlite3_str_new(unsafe { (*p).db }) };
                        n_sep =
                            unsafe {
                                    strlen(unsafe { (*p).spec.z_column_sep } as *const i8)
                                } as i32;
                        mx_w =
                            unsafe { (*p).mx_width } -
                                (n_sep + unsafe { (*p).u.s_line.mx_col_wth });
                        b_ww =
                            (unsafe { (*p).spec.b_word_wrap } as i32 == 2) as i32;
                        {
                            i = 0;
                            '__b91: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b91; }
                                '__c91: loop {
                                    let mut z_val: *const i8 = core::ptr::null();
                                    let mut cnt: i32 = 0;
                                    qrf_width_print(p as *const Qrf, unsafe { (*p).p_out },
                                        -unsafe { (*p).u.s_line.mx_col_wth },
                                        unsafe {
                                                *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                            } as *const i8);
                                    unsafe {
                                        sqlite3_str_append(unsafe { (*p).p_out },
                                            unsafe { (*p).spec.z_column_sep } as *const i8, n_sep)
                                    };
                                    qrf_render_value(p, p_val, i);
                                    z_val = unsafe { sqlite3_str_value(p_val) } as *const i8;
                                    if z_val == core::ptr::null() {
                                        z_val = c"".as_ptr() as *mut i8 as *const i8;
                                    }
                                    '__b92: loop {
                                        '__c92: loop {
                                            let mut n_this: i32 = 0;
                                            let mut n_wide: i32 = 0;
                                            let mut i_next: i32 = 0;
                                            qrf_wrap_line(z_val, mx_w, b_ww, &mut n_this, &mut n_wide,
                                                &mut i_next);
                                            if cnt != 0 {
                                                unsafe {
                                                    sqlite3_str_appendchar(unsafe { (*p).p_out },
                                                        unsafe { (*p).u.s_line.mx_col_wth } + n_sep,
                                                        ' ' as i32 as i8)
                                                };
                                            }
                                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                                            if cnt > unsafe { (*p).mx_height } {
                                                z_val = c"...".as_ptr() as *mut i8 as *const i8;
                                                n_this = { i_next = 3; i_next };
                                            }
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out }, z_val, n_this)
                                            };
                                            unsafe {
                                                sqlite3_str_append(unsafe { (*p).p_out },
                                                    c"\n".as_ptr() as *mut i8 as *const i8, 1)
                                            };
                                            {
                                                let __n = i_next;
                                                let __p = &mut z_val;
                                                *__p = unsafe { (*__p).offset(__n as isize) };
                                            };
                                            break '__c92;
                                        }
                                        if !(unsafe { *z_val.offset(0 as isize) } != 0) {
                                            break '__b92;
                                        }
                                    }
                                    unsafe { sqlite3_str_reset(p_val) };
                                    break '__c91;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        qrf_str_err(p, p_val);
                        unsafe {
                            sqlite3_free(unsafe { sqlite3_str_finish(p_val) } as
                                    *mut ())
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                5 => {
                    {
                        let mut z_eqp_line: *const i8 =
                            unsafe { sqlite3_column_text(unsafe { (*p).p_stmt }, 3) } as
                                *const i8;
                        let i_eqp_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 0) };
                        let i_parent_id: i32 =
                            unsafe { sqlite3_column_int(unsafe { (*p).p_stmt }, 1) };
                        if z_eqp_line == core::ptr::null() {
                            z_eqp_line = c"".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { *z_eqp_line.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            qrf_eqp_render(p, 0 as i64);
                        }
                        qrf_eqp_append(p, i_eqp_id, i_parent_id, z_eqp_line);
                        break '__s85;
                    }
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
                _ => {
                    {
                        if unsafe { (*p).n_row } == 0 as i64 &&
                                unsafe { (*p).spec.b_titles } as i32 == 2 {
                            let saved_e_text: i32 = unsafe { (*p).spec.e_text } as i32;
                            unsafe { (*p).spec.e_text = unsafe { (*p).spec.e_title } };
                            {
                                i = 0;
                                '__b93: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b93; }
                                    '__c93: loop {
                                        let z_c_name_3: *const i8 =
                                            unsafe { sqlite3_column_name(unsafe { (*p).p_stmt }, i) };
                                        if i > 0 {
                                            unsafe {
                                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                                    unsafe { (*p).spec.z_column_sep } as *const i8)
                                            };
                                        }
                                        qrf_encode_text(p, unsafe { (*p).p_out }, z_c_name_3);
                                        break '__c93;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_appendall(unsafe { (*p).p_out },
                                    unsafe { (*p).spec.z_row_sep } as *const i8)
                            };
                            qrf_write(p);
                            unsafe { (*p).spec.e_text = saved_e_text as u8 };
                        }
                        {
                            i = 0;
                            '__b94: loop {
                                if !(i < unsafe { (*p).n_col }) { break '__b94; }
                                '__c94: loop {
                                    if i > 0 {
                                        unsafe {
                                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                                unsafe { (*p).spec.z_column_sep } as *const i8)
                                        };
                                    }
                                    qrf_render_value(p, unsafe { (*p).p_out }, i);
                                    break '__c94;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_str_appendall(unsafe { (*p).p_out },
                                unsafe { (*p).spec.z_row_sep } as *const i8)
                        };
                        qrf_write(p);
                        break '__s85;
                    }
                }
            }
        }
        {
            let __p = unsafe { &mut (*p).n_row };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
}

///* Reset the prepared statement.
extern "C" fn qrf_reset_stmt(p: *mut Qrf) -> () {
    let rc: i32 = unsafe { sqlite3_reset(unsafe { (*p).p_stmt }) };
    if rc != 0 && unsafe { (*p).i_err } == 0 {
        unsafe {
            qrf_error(unsafe { &mut *p }, rc,
                c"%s".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
        };
    }
}

///* Finish rendering the results
extern "C" fn qrf_finalize(p: *mut Qrf) -> () {
    unsafe {
        '__s95:
            {
            match unsafe { (*p).spec.e_style } {
                3 => {
                    {
                        unsafe {
                            sqlite3_str_appendf(unsafe { (*p).p_out },
                                c"%lld\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p).n_row })
                        };
                        break '__s95;
                    }
                    {
                        if unsafe { (*p).n_row } > 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}]\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if unsafe { (*p).n_row } > 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}\n".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if unsafe { (*p).u.n_ins } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c";\n".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if !(unsafe { (*p).u.s_line.az_col }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b96: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b96; }
                                    '__c96: loop {
                                        unsafe {
                                            sqlite3_free(unsafe {
                                                        *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                    } as *mut ())
                                        };
                                        break '__c96;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_free(unsafe { (*p).u.s_line.az_col } as *mut ())
                            };
                        }
                        break '__s95;
                    }
                    {
                        let n_cycle: i64 = 0 as i64;
                        qrf_eqp_render(p, n_cycle);
                        break '__s95;
                    }
                    { qrf_eqp_render(p, 0 as i64); break '__s95; }
                }
                9 => {
                    {
                        if unsafe { (*p).n_row } > 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}]\n".as_ptr() as *mut i8 as *const i8, 3)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if unsafe { (*p).n_row } > 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}\n".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if unsafe { (*p).u.n_ins } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c";\n".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if !(unsafe { (*p).u.s_line.az_col }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b96: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b96; }
                                    '__c96: loop {
                                        unsafe {
                                            sqlite3_free(unsafe {
                                                        *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                    } as *mut ())
                                        };
                                        break '__c96;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_free(unsafe { (*p).u.s_line.az_col } as *mut ())
                            };
                        }
                        break '__s95;
                    }
                    {
                        let n_cycle: i64 = 0 as i64;
                        qrf_eqp_render(p, n_cycle);
                        break '__s95;
                    }
                    { qrf_eqp_render(p, 0 as i64); break '__s95; }
                }
                10 => {
                    {
                        if unsafe { (*p).n_row } > 0 as i64 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c"}\n".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if unsafe { (*p).u.n_ins } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c";\n".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if !(unsafe { (*p).u.s_line.az_col }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b96: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b96; }
                                    '__c96: loop {
                                        unsafe {
                                            sqlite3_free(unsafe {
                                                        *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                    } as *mut ())
                                        };
                                        break '__c96;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_free(unsafe { (*p).u.s_line.az_col } as *mut ())
                            };
                        }
                        break '__s95;
                    }
                    {
                        let n_cycle: i64 = 0 as i64;
                        qrf_eqp_render(p, n_cycle);
                        break '__s95;
                    }
                    { qrf_eqp_render(p, 0 as i64); break '__s95; }
                }
                8 => {
                    {
                        if unsafe { (*p).u.n_ins } != 0 {
                            unsafe {
                                sqlite3_str_append(unsafe { (*p).p_out },
                                    c";\n".as_ptr() as *mut i8 as *const i8, 2)
                            };
                        }
                        break '__s95;
                    }
                    {
                        if !(unsafe { (*p).u.s_line.az_col }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b96: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b96; }
                                    '__c96: loop {
                                        unsafe {
                                            sqlite3_free(unsafe {
                                                        *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                    } as *mut ())
                                        };
                                        break '__c96;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_free(unsafe { (*p).u.s_line.az_col } as *mut ())
                            };
                        }
                        break '__s95;
                    }
                    {
                        let n_cycle: i64 = 0 as i64;
                        qrf_eqp_render(p, n_cycle);
                        break '__s95;
                    }
                    { qrf_eqp_render(p, 0 as i64); break '__s95; }
                }
                11 => {
                    {
                        if !(unsafe { (*p).u.s_line.az_col }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b96: loop {
                                    if !(i < unsafe { (*p).n_col }) { break '__b96; }
                                    '__c96: loop {
                                        unsafe {
                                            sqlite3_free(unsafe {
                                                        *unsafe { (*p).u.s_line.az_col.offset(i as isize) }
                                                    } as *mut ())
                                        };
                                        break '__c96;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_free(unsafe { (*p).u.s_line.az_col } as *mut ())
                            };
                        }
                        break '__s95;
                    }
                    {
                        let n_cycle: i64 = 0 as i64;
                        qrf_eqp_render(p, n_cycle);
                        break '__s95;
                    }
                    { qrf_eqp_render(p, 0 as i64); break '__s95; }
                }
                16 => {
                    {
                        let n_cycle: i64 = 0 as i64;
                        qrf_eqp_render(p, n_cycle);
                        break '__s95;
                    }
                    { qrf_eqp_render(p, 0 as i64); break '__s95; }
                }
                17 => {
                    {
                        let n_cycle: i64 = 0 as i64;
                        qrf_eqp_render(p, n_cycle);
                        break '__s95;
                    }
                    { qrf_eqp_render(p, 0 as i64); break '__s95; }
                }
                5 => { { qrf_eqp_render(p, 0 as i64); break '__s95; } }
                _ => {}
            }
        }
        qrf_write(p);
        qrf_str_err(p, unsafe { (*p).p_out });
        if !(unsafe { (*p).spec.pz_output }).is_null() {
            if !(unsafe {
                                *unsafe { (*p).spec.pz_output.offset(0 as isize) }
                            }).is_null() {
                let mut n: Sqlite3Int64 = 0 as Sqlite3Int64;
                let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
                let mut z_combined: *mut i8 = core::ptr::null_mut();
                sz =
                    unsafe {
                            strlen(unsafe {
                                        *unsafe { (*p).spec.pz_output.offset(0 as isize) }
                                    } as *const i8)
                        } as Sqlite3Int64;
                n =
                    unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } as
                        Sqlite3Int64;
                z_combined =
                    unsafe {
                            sqlite3_realloc64(unsafe {
                                        *unsafe { (*p).spec.pz_output.offset(0 as isize) }
                                    } as *mut (), (sz + n + 1 as Sqlite3Int64) as Sqlite3Uint64)
                        } as *mut i8;
                if z_combined == core::ptr::null_mut() {
                    unsafe {
                        sqlite3_free(unsafe {
                                    *unsafe { (*p).spec.pz_output.offset(0 as isize) }
                                } as *mut ())
                    };
                    unsafe {
                        *unsafe { (*p).spec.pz_output.offset(0 as isize) } =
                            core::ptr::null_mut()
                    };
                    qrf_oom(p);
                } else {
                    unsafe {
                        *unsafe { (*p).spec.pz_output.offset(0 as isize) } =
                            z_combined
                    };
                    unsafe {
                        memcpy(unsafe { z_combined.offset(sz as isize) } as *mut (),
                            unsafe { sqlite3_str_value(unsafe { (*p).p_out }) } as
                                *const (), (n + 1 as Sqlite3Int64) as u64)
                    };
                }
                unsafe {
                    sqlite3_free(unsafe {
                                sqlite3_str_finish(unsafe { (*p).p_out })
                            } as *mut ())
                };
            } else {
                unsafe {
                    *unsafe { (*p).spec.pz_output.offset(0 as isize) } =
                        unsafe { sqlite3_str_finish(unsafe { (*p).p_out }) }
                };
            }
        } else if !(unsafe { (*p).p_out }).is_null() {
            unsafe {
                sqlite3_free(unsafe {
                            sqlite3_str_finish(unsafe { (*p).p_out })
                        } as *mut ())
            };
        }
        if unsafe { (*p).exp_mode } > 0 {
            unsafe {
                sqlite3_stmt_explain(unsafe { (*p).p_stmt },
                    unsafe { (*p).exp_mode } - 1)
            };
        }
        if !(unsafe { (*p).actual_width }).is_null() {
            unsafe { sqlite3_free(unsafe { (*p).actual_width } as *mut ()) };
        }
        if !(unsafe { (*p).p_j_trans }).is_null() {
            let db: *mut Sqlite3 =
                unsafe { sqlite3_db_handle(unsafe { (*p).p_j_trans }) };
            unsafe { sqlite3_finalize(unsafe { (*p).p_j_trans }) };
            unsafe { sqlite3_close(db) };
        }
    }
}

///* Interfaces
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_format_query_result(p_stmt: *mut Sqlite3Stmt,
    p_spec: *const Sqlite3QrfSpec, pz_err: *mut *mut i8) -> i32 {
    let mut qrf: Qrf = unsafe { core::mem::zeroed() };
    if p_stmt == core::ptr::null_mut() { return 0; }
    if p_spec == core::ptr::null() { return 21; }
    if unsafe { sqlite3_stmt_busy(p_stmt) } != 0 { return 5; }
    qrf_initialize(&mut qrf, p_stmt, p_spec, pz_err);
    '__s97:
        {
        match qrf.spec.e_style {
            1 => {
                {

                    /// Columnar modes require that the entire query be evaluated and the
                    ///* results stored in memory, so that we can compute column widths
                    qrf_columnar(&mut qrf);
                    break '__s97;
                }
                { qrf_explain(&mut qrf); break '__s97; }
                { qrf_scan_status_vm(&mut qrf); break '__s97; }
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            2 => {
                {

                    /// Columnar modes require that the entire query be evaluated and the
                    ///* results stored in memory, so that we can compute column widths
                    qrf_columnar(&mut qrf);
                    break '__s97;
                }
                { qrf_explain(&mut qrf); break '__s97; }
                { qrf_scan_status_vm(&mut qrf); break '__s97; }
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            13 => {
                {

                    /// Columnar modes require that the entire query be evaluated and the
                    ///* results stored in memory, so that we can compute column widths
                    qrf_columnar(&mut qrf);
                    break '__s97;
                }
                { qrf_explain(&mut qrf); break '__s97; }
                { qrf_scan_status_vm(&mut qrf); break '__s97; }
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            19 => {
                {

                    /// Columnar modes require that the entire query be evaluated and the
                    ///* results stored in memory, so that we can compute column widths
                    qrf_columnar(&mut qrf);
                    break '__s97;
                }
                { qrf_explain(&mut qrf); break '__s97; }
                { qrf_scan_status_vm(&mut qrf); break '__s97; }
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            6 => {
                { qrf_explain(&mut qrf); break '__s97; }
                { qrf_scan_status_vm(&mut qrf); break '__s97; }
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            18 => {
                { qrf_scan_status_vm(&mut qrf); break '__s97; }
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            16 => {
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            17 => {
                { qrf_eqp_stats(&mut qrf); break '__s97; }
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
            _ => {
                {
                    while qrf.i_err == 0 &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        qrf_one_simple_row(&mut qrf);
                    }
                    break '__s97;
                }
            }
        }
    }
    qrf_reset_stmt(&mut qrf);
    qrf_finalize(&mut qrf);
    return qrf.i_err;
}

static z_space: [i8; 6] =
    [32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 0 as i8];

static mx_w_1: i32 = 10000000 as i32;

static a_explain_width: [i32; 8] = [4, 13, 4, 4, 4, 13, 2, 13];

static a_explain_map: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

static a_scan_exp_width: [i32; 10] = [4, 15, 6, 13, 4, 4, 4, 13, 2, 13];

static a_scan_exp_map: [i32; 10] = [0, 9, 8, 1, 2, 3, 4, 5, 6, 7];

static mut z_sql: *const i8 =
    c"  SELECT addr, opcode, p1, p2, p3, p4, p5, comment, nexec,   format(\'% 6s (%.2f%%)\',      CASE WHEN ncycle<100_000 THEN ncycle || \' \'         WHEN ncycle<100_000_000 THEN (ncycle/1_000) || \'K\'         WHEN ncycle<100_000_000_000 THEN (ncycle/1_000_000) || \'M\'         ELSE (ncycle/1000_000_000) || \'G\' END,       ncycle*100.0/(sum(ncycle) OVER ())   )  AS cycles   FROM bytecode(?1)".as_ptr()
            as *mut i8 as *const i8;

static a_suffix: [i8; 6] =
    ['K' as i32 as i8, 'M' as i32 as i8, 'G' as i32 as i8, 'T' as i32 as i8,
            'P' as i32 as i8, 'E' as i32 as i8];

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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
